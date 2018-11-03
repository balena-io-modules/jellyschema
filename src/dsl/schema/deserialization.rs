use crate::dsl::object_types::deserialization::deserialize_object_type;
use crate::dsl::schema::Property;
use crate::dsl::schema::PropertyEntry;
use crate::dsl::schema::PropertyList;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde_yaml::Mapping;
use serde_yaml::Sequence;
use serde_yaml::Value;

pub fn deserialize_property_list<'de, D>(deserializer: D) -> Result<Option<PropertyList>, D::Error>
where
    D: Deserializer<'de>,
{
    let maybe_sequence: Option<serde_yaml::Sequence> = Option::deserialize(deserializer)?;
    match maybe_sequence {
        Some(sequence) => Ok(Some(sequence_to_property_list(sequence)?)),
        None => Ok(None),
    }
}

fn sequence_to_property_list<E>(sequence: Sequence) -> Result<PropertyList, E>
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

    Ok(PropertyList {
        entries: list,
        property_names: property_names.clone(),
    })
}

fn deserialize_property<E>(value: &Value) -> Result<Property, E>
where
    E: Error,
{
    let key = Value::from("type");
    let mapping = value
        .as_mapping()
        .ok_or(Error::custom("property is not a yaml mapping"))?;
    let type_information = deserialize_object_type(&mapping)?;

    let display_information = serde_yaml::from_value(value.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize display information - {}", e)))?;
    Ok(Property {
        type_information,
        display_information,
    })
}

fn mapping_to_property_entry<E>(mapping: &Mapping) -> Result<PropertyEntry, E>
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
    Ok(PropertyEntry {
        name: key,
        property: value,
    })
}
