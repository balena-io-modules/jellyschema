use crate::dsl::compiler::Property;
use crate::dsl::compiler::PropertyEntry;
use crate::dsl::compiler::PropertyList;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_yaml::Mapping;
use serde_yaml::Sequence;

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
            .ok_or(Error::custom("cannot deserialize property as mapping"))?;
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

fn mapping_to_property_entry<E>(mapping: &Mapping) -> Result<PropertyEntry, E>
where
    E: Error,
{
    let (key, value) = mapping
        .into_iter()
        .next()
        .ok_or(Error::custom("cannot get first element of the sequence"))?;
    let key: String =
        serde_yaml::from_value(key.clone()).map_err(|e| Error::custom(format!("cannot deserialize key - {:?}", e)))?;
    let value: Property = serde_yaml::from_value(value.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize value - {:?}", e)))?;
    Ok(PropertyEntry {
        name: key,
        property: value,
    })
}
