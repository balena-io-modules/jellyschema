use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::TypeDefinition;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde_derive::Deserialize;
use serde_yaml::Mapping;
use serde_yaml::Sequence;

#[derive(Clone, Debug, Deserialize)]
pub struct SourceSchema {
    pub title: String,
    pub version: u64,
    #[serde(default, deserialize_with = "crate::dsl::schema::deserialize_property_list")]
    #[serde(rename = "properties")]
    pub property_list: Option<PropertyList>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Property {
    #[serde(flatten)]
    pub type_information: TypeDefinition,
    #[serde(flatten)]
    pub display_information: DisplayInformation,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct DisplayInformation {
    pub title: Option<String>,
    pub help: Option<String>,
    pub warning: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PropertyEntry {
    pub name: String,
    pub property: Property,
}

#[derive(Clone, Debug)]
pub struct PropertyList {
    property_names: Vec<String>,
    pub entries: Vec<PropertyEntry>,
}

impl PropertyList {
    pub fn property_names(&self) -> Vec<&str> {
        self.property_names.iter().map(|name| name.as_str()).collect()
    }

    pub fn required_property_names(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter_map(
                |property_entry| match &property_entry.property.type_information.r#type {
                    Some(type_spec) => match type_spec {
                        ObjectType::Required(_) => Some(property_entry.name.as_str()),
                        ObjectType::Optional(_) => None,
                    },
                    None => None,
                },
            )
            .collect()
    }
}

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
    let value: Property = serde_yaml::from_value(value.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize property value '{:?}' - {}", value, e)))?;
    Ok(PropertyEntry {
        name: key,
        property: value,
    })
}
