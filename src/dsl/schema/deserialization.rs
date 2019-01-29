use balena_temen::ast::Expression;
use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

use crate::dsl::schema::Annotations;
use crate::dsl::schema::compiler::CompilationError;
use crate::dsl::schema::DocumentRoot;
use crate::dsl::schema::dynamic::keys_values;
use crate::dsl::schema::NamedSchema;
use crate::dsl::schema::object_types::deserialization::deserialize_individual_type_definition;
use crate::dsl::schema::object_types::deserialization::deserialize_object_type;
use crate::dsl::schema::object_types::ObjectType;
use crate::dsl::schema::object_types::RawObjectType;
use crate::dsl::schema::Schema;
use crate::dsl::schema::SchemaList;
use crate::dsl::schema::when::dependencies_for_schema_list;
use crate::dsl::schema::when::DependencyGraph;
use crate::dsl::schema::Widget;

pub fn deserialize_root(schema: &Value) -> Result<DocumentRoot, CompilationError> {
    let maybe_root = schema.as_mapping();
    const DEFAULT_VERSION: u64 = 1;
    let version = match maybe_root {
        Some(mapping) => Ok({
            let version = mapping.get(&Value::from("version"));

            match version {
                Some(version) => Some(
                    version
                        .as_u64()
                        .ok_or_else(|| CompilationError::with_message("version must be a positive integer"))?,
                ),
                None => None,
            }
        }),
        None => Err(CompilationError::with_message(
            "root level schema needs to be a yaml mapping",
        )),
    }?
    .unwrap_or(DEFAULT_VERSION);

    if version != DEFAULT_VERSION {
        return Err(CompilationError::with_message(&format!(
            "invalid version number '{}' specified",
            version
        )));
    }

    let schema = deserialize_schema::<serde_yaml::Error>(&schema)?;

    // this is recursive already, should get the whole tree for all children schemas
    let dependencies = dependencies_for_schema_list(schema.children.as_ref(), DependencyGraph::empty())?;

    Ok(DocumentRoot {
        version,
        schema: Some(schema),
        dependencies: Some(dependencies),
    })
}

pub fn deserialize_schema<E>(value: &Value) -> Result<Schema, E>
where
    E: Error,
{
    let yaml_mapping = value
        .as_mapping()
        .ok_or_else(|| Error::custom(format!("schema is not a yaml mapping - {:#?}", value)))?;
    let type_information = type_information(&yaml_mapping)?;

    let annotations = annotations(value)?;
    let annotations = annotations_from_type(annotations, &type_information);
    let properties = properties(yaml_mapping)?;
    let dynamic = keys_values(yaml_mapping)?;

    let when = when(yaml_mapping)?;
    let formula = formula(yaml_mapping)?;

    Ok(Schema {
        object_type: type_information,
        children: properties,
        dynamic,
        annotations,
        when,
        formula,
    })
}

fn formula<E>(yaml_mapping: &Mapping) -> Result<Option<String>, E>
where
    E: Error,
{
    let value = yaml_mapping.get(&Value::from("formula"));
    match value {
        None => Ok(None),
        Some(value) => match value {
            Value::String(string) => Ok(Some(string.to_string())),
            _ => {
                let string = serde_json::to_string(value)
                    .map_err(|e| Error::custom(format!("error parsing formula value expression: {}", e)))?;
                Ok(Some(string))
            }
        },
    }
}

fn when<E>(yaml_mapping: &Mapping) -> Result<Option<Expression>, E>
where
    E: Error,
{
    let when = yaml_mapping.get(&Value::from("when"));
    match when {
        None => Ok(None),
        Some(mapping) => match mapping {
            Value::String(string) => {
                Ok(Some(string.parse().map_err(|e| {
                    Error::custom(format!("error parsing when expression: {}", e))
                })?))
            }
            _ => Err(Error::custom(format!("unknown shape of `when`: {:#?}", mapping))),
        },
    }
}

fn properties<E>(yaml_mapping: &Mapping) -> Result<Option<SchemaList>, E>
where
    E: Error,
{
    let properties = yaml_mapping.get(&Value::from("properties"));
    let properties = match properties {
        None => None,
        Some(properties) => match properties {
            Value::Sequence(sequence) => Some(sequence_to_schema_list(&sequence.to_vec())?),
            _ => return Err(Error::custom("`properties` is not a yaml sequence")),
        },
    };
    Ok(properties)
}

fn annotations<E>(value: &Value) -> Result<Annotations, E>
where
    E: Error,
{
    let annotations = serde_yaml::from_value(value.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize schema annotations - {}", e)))?;
    Ok(annotations)
}

fn type_information<E>(yaml_mapping: &Mapping) -> Result<ObjectType, E>
where
    E: Error,
{
    let type_information = deserialize_object_type(&yaml_mapping)?;

    if let Some(type_information) = type_information {
        Ok(type_information)
    } else {
        let type_definition = deserialize_individual_type_definition("object", yaml_mapping)?;
        Ok(type_definition)
    }
}

fn annotations_from_type(old_annotations: Annotations, type_information: &ObjectType) -> Annotations {
    let mut widget = old_annotations.widget;
    if let RawObjectType::Text(_) = type_information.inner_raw() {
        widget = Some(Widget::Textarea)
    }
    Annotations {
        widget,
        ..old_annotations
    }
}

fn sequence_to_schema_list<E>(sequence: &[Value]) -> Result<SchemaList, E>
where
    E: Error,
{
    let list_of_maybe_entries = sequence.iter().map(|value| {
        let mapping = value
            .as_mapping()
            .ok_or_else(|| Error::custom(format!("cannot deserialize schema {:#?} as mapping", value)))?;
        Ok(mapping_to_named_schema(mapping)?)
    });

    let list: Result<Vec<_>, E> = list_of_maybe_entries.collect();
    let list = list?;

    Ok(SchemaList { entries: list })
}

fn mapping_to_named_schema<E>(mapping: &Mapping) -> Result<NamedSchema, E>
where
    E: Error,
{
    let (key, value) = mapping
        .into_iter()
        .next()
        .ok_or_else(|| Error::custom("cannot get first element of the sequence"))?;
    let key: String = serde_yaml::from_value(key.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize named schema name - {}", e)))?;
    let value = deserialize_schema(&value)?;
    Ok(NamedSchema {
        name: key,
        schema: value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fails_on_unknown_version_number() {
        let schema = serde_yaml::from_str(
            r#"
        version: 2
        "#,
        )
        .unwrap();

        let deserialized: Result<DocumentRoot, CompilationError> = deserialize_root(&schema);

        assert!(deserialized.err().is_some());
    }

    #[test]
    fn passes_on_known_version_number() {
        let schema = serde_yaml::from_str(
            r#"
        version: 1
        "#,
        )
        .unwrap();

        let deserialized: Result<DocumentRoot, CompilationError> = deserialize_root(&schema);
        assert!(deserialized.ok().is_some());
    }
}
