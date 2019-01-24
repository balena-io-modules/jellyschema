use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

use crate::dsl::schema::compiler::CompilationError;
use crate::dsl::schema::DocumentRoot;
use crate::dsl::schema::NamedSchema;
use crate::dsl::schema::object_types::deserialization::deserialize_object_type;
use crate::dsl::schema::object_types::ObjectType;
use crate::dsl::schema::object_types::RawObjectType;
use crate::dsl::schema::Schema;
use crate::dsl::schema::SchemaList;
use crate::dsl::schema::when::dependencies_for_schema_list;
use crate::dsl::schema::when::DependencyGraph;
use crate::dsl::schema::Annotations;
use crate::dsl::schema::Widget;
use crate::dsl::schema::object_types::ObjectTypeData;
use balena_temen::ast::Expression;
use crate::dsl::schema::KeysValues;
use crate::dsl::schema::KeysSchema;
use regex::Regex;

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

    let mapping = mapping(yaml_mapping)?;
    let when = when(yaml_mapping)?;
    let formula = formula(yaml_mapping)?;

    Ok(Schema {
        object_type: type_information,
        children: properties,
        mapping: mapping.cloned(),
        dynamic,
        annotations,
        when,
        formula,
    })
}

fn deserialize_keys_schema<E>(value: &Value) -> Result<KeysSchema, E>
where E: Error
{

    let yaml_mapping = value
        .as_mapping()
        .ok_or_else(|| Error::custom(format!("schema is not a yaml mapping - {:#?}", value)))?;


    let pattern = yaml_mapping.get(&Value::from("pattern"));
    let title = yaml_mapping.get(&Value::from("title"));
    let type_spec = yaml_mapping.get(&Value::from("type"));

    if type_spec.is_none() {
        return Err(Error::custom("`keys` must have a `type` specified"))
    }

    let type_spec = type_spec.unwrap().as_str();
    if type_spec.is_none() {
        return Err(Error::custom("`keys` must have `type` specified as string"))
    }

    let title = match title {
        None => Ok(None),
        Some(title) => match title.as_str() {
            None => Err(Error::custom("`title` must be a string")),
            Some(title) => Ok(Some(title.to_string()))
        }
    }?;

    if pattern.is_none() {
        return Err(Error::custom("`keys` must have a `pattern`"))
    }

    let pattern = pattern.unwrap().as_str();
    if pattern.is_none() {
        return Err(Error::custom("`pattern` must be a string"))
    }

    let pattern = Regex::new(pattern.unwrap()).map_err(|e| Error::custom("`pattern` is not a regex"))?;

    Ok(KeysSchema::new(pattern, title))
}

fn keys_values<E>(yaml_mapping: &Mapping) -> Result<Option<Box<KeysValues>>, E>
where E: Error
{
    let key = yaml_mapping.get(&Value::from("keys"));
    let value = yaml_mapping.get(&Value::from("values"));

    if key.is_none() && value.is_none() {
        return Ok(None);
    }

    if key.is_some() && value.is_none() {
        return Err(Error::custom("need `values` when specifying a `keys`"));
    }

    if key.is_none() && value.is_some() {
        return Err(Error::custom("need `keys` when specifying a `values`"));
    }

    let key = deserialize_keys_schema(key.unwrap())?;
    let value = deserialize_schema(value.unwrap())?;

    Ok(Some(Box::new(KeysValues::new(key, value))))
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

fn mapping<E>(yaml_mapping: &Mapping) -> Result<Option<&Mapping>, E>
where
    E: Error,
{
    let mapping = yaml_mapping.get(&Value::from("mapping"));
    let mapping = match mapping {
        None => Ok(None),
        Some(mapping) => match mapping {
            Value::Mapping(mapping) => Ok(Some(mapping)),
            _ => Err(Error::custom(format!("cannot deserialize mapping {:#?}", mapping))),
        },
    }?;
    Ok(mapping)
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

fn type_information<E>(yaml_mapping: &Mapping) -> Result<Option<ObjectType>, E>
where
    E: Error,
{
    let mut type_information = deserialize_object_type(&yaml_mapping)?;
    if type_information.is_none() {
        let raw_type = RawObjectType::Object;
        let type_data = ObjectTypeData::with_raw_type(raw_type);
        type_information = Some(ObjectType::Required(type_data));
    }
    Ok(type_information)
}

fn annotations_from_type(old_annotations: Annotations, type_information: &Option<ObjectType>) -> Annotations {
    let mut widget = old_annotations.widget;
    if let Some(type_info) = type_information {
        if let RawObjectType::Text(_) = type_info.inner_raw() {
            widget = Some(Widget::Textarea)
        }
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
