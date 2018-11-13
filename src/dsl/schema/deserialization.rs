use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

use crate::dsl::schema::compiler::CompilationError;
use crate::dsl::schema::NamedProperty;
use crate::dsl::schema::object_types::deserialization::deserialize_object_type;
use crate::dsl::schema::object_types::ObjectType;
use crate::dsl::schema::object_types::RawObjectType;
use crate::dsl::schema::Property;
use crate::dsl::schema::PropertyList;
use crate::dsl::schema::SchemaRoot;

pub fn deserialize_root<E>(schema: &Value) -> Result<SchemaRoot, CompilationError>
where
    E: serde::de::Error,
{
    let maybe_root = schema.as_mapping();
    let version = match maybe_root {
        Some(mapping) => Ok({
            let version = mapping
                .get(&Value::from("version"))
                .ok_or_else(|| CompilationError::with_message("you must specify schema version"))?;
            version
                .as_u64()
                .ok_or_else(|| CompilationError::with_message("version must be a positive integer"))?
        }),
        None => Err(CompilationError::with_message(
            "root level schema needs to be a yaml mapping",
        )),
    }?;
    let self_property = Some(deserialize_property::<serde_yaml::Error>(&schema)?);
    Ok(SchemaRoot { version, self_property })
}

pub fn sequence_to_property_list<E>(sequence: &[Value]) -> Result<PropertyList, E>
where
    E: Error,
{
    let mut property_names = vec![];
    let list_of_maybe_entries = sequence.into_iter().map(|value| {
        let mapping = value
            .as_mapping()
            .ok_or_else(|| Error::custom("cannot deserialize property as mapping"))?;
        let property_entry = mapping_to_property_entry(mapping)?;
        property_names.push(property_entry.name.clone());
        Ok(property_entry)
    });

    let list: Result<Vec<_>, E> = list_of_maybe_entries.collect();
    let list = list?;

    Ok(PropertyList { entries: list })
}

fn mapping_to_property_entry<E>(mapping: &Mapping) -> Result<NamedProperty, E>
where
    E: Error,
{
    let (key, value) = mapping
        .into_iter()
        .next()
        .ok_or_else(|| Error::custom("cannot get first element of the sequence"))?;
    let key: String = serde_yaml::from_value(key.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize property key - {}", e)))?;
    let value = deserialize_property(&value)?;
    Ok(NamedProperty {
        name: key,
        property: value,
    })
}

pub fn deserialize_property<E>(value: &Value) -> Result<Property, E>
where
    E: Error,
{
    let yaml_mapping = value
        .as_mapping()
        .ok_or_else(|| Error::custom(format!("property is not a yaml mapping - {:#?}", value)))?;
    let mut type_information = deserialize_object_type(&yaml_mapping)?;

    if type_information.is_none() {
        type_information = Some(vec![ObjectType::Required(RawObjectType::Object)]);
    }

    let display_information = serde_yaml::from_value(value.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize display information - {}", e)))?;

    let properties = yaml_mapping.get(&Value::from("properties"));
    let properties = match properties {
        None => None,
        Some(properties) => match properties {
            Value::Sequence(sequence) => Some(sequence_to_property_list(&sequence.to_vec())?),
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

    Ok(Property {
        types: type_information,
        annotations: display_information,
        children: properties,
        mapping: mapping.cloned(),
    })
}
