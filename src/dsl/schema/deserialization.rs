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

    eprintln!("{:#?}", dependencies);

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
    let mut type_information = deserialize_object_type(&yaml_mapping)?;

    if type_information.is_none() {
        type_information = Some(vec![ObjectType::Required(RawObjectType::Object)]);
    }

    let annotations = serde_yaml::from_value(value.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize schema annotations - {}", e)))?;

    let properties = yaml_mapping.get(&Value::from("properties"));
    let properties = match properties {
        None => None,
        Some(properties) => match properties {
            Value::Sequence(sequence) => Some(sequence_to_schema_list(&sequence.to_vec())?),
            _ => return Err(Error::custom("`properties` is not a yaml sequence")),
        },
    };

    let mapping = yaml_mapping.get(&Value::from("mapping"));
    let mapping = match mapping {
        None => Ok(None),
        Some(mapping) => match mapping {
            Value::Mapping(mapping) => Ok(Some(mapping)),
            _ => Err(Error::custom(format!("cannot deserialize mapping {:#?}", mapping))),
        },
    }?;

    let when = yaml_mapping.get(&Value::from("when"));
    let when = match when {
        None => Ok(None),
        Some(mapping) => match mapping {
            Value::String(string) => {
                Ok(Some(string.parse().map_err(|e| {
                    Error::custom(format!("error parsing when expression: {}", e))
                })?))
            }
            _ => Err(Error::custom(format!("unknown shape of `when`: {:#?}", mapping))),
        },
    };
    Ok(Schema {
        types: type_information,
        annotations,
        children: properties,
        mapping: mapping.cloned(),
        when: when?,
    })
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
