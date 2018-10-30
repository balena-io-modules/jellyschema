mod deserialization;
mod normalization;
mod validation;

use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::TypeDefinition;
use serde_derive::Deserialize;

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
