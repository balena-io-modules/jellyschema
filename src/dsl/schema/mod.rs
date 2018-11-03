mod deserialization;
mod normalization;
mod validation;

use crate::dsl::object_types::ObjectType;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SourceSchema {
    pub title: String,
    pub version: u64,
    #[serde(
        default,
        deserialize_with = "crate::dsl::schema::deserialization::deserialize_property_list"
    )]
    #[serde(rename = "properties")]
    pub property_list: Option<PropertyList>,
}

#[derive(Clone, Debug)]
pub struct PropertyList {
    property_names: Vec<String>,
    pub entries: Vec<PropertyEntry>,
}

// fixme move serialize implementation to the `ui_configuration` module
#[derive(Clone, Debug, Serialize)]
pub struct PropertyEntry {
    pub name: String,
    #[serde(flatten)]
    pub property: Property,
}

#[derive(Clone, Debug)]
pub struct Property {
    pub type_information: Option<ObjectType>,
    pub display_information: DisplayInformation,
    pub property_list: Option<PropertyList>,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct DisplayInformation {
    pub title: Option<String>,
    pub help: Option<String>,
    pub warning: Option<String>,
    pub description: Option<String>,
}

impl PropertyList {
    pub fn property_names(&self) -> Vec<&str> {
        self.property_names.iter().map(|name| name.as_str()).collect()
    }

    pub fn required_property_names(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter_map(|property_entry| match &property_entry.property.type_information {
                Some(type_spec) => match type_spec {
                    ObjectType::Required(_) => Some(property_entry.name.as_str()),
                    ObjectType::Optional(_) => None,
                },
                None => None,
            })
            .collect()
    }
}
