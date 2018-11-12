pub mod deserialization;

use crate::dsl::object_types::ObjectType;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Clone, Debug)]
pub struct SourceSchema {
    pub version: u64,
    pub self_property: Option<Property>,
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
    pub types: Option<Vec<ObjectType>>,
    pub display_information: DisplayInformation,
    pub property_list: Option<PropertyList>,
    pub mapping: Option<serde_yaml::Mapping>, // TODO: real mapping support
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
            .filter_map(|property_entry| match &property_entry.property.types {
                Some(type_list) => {
                    if type_list.iter().any(|type_spec| match type_spec {
                        ObjectType::Required(_) => true,
                        ObjectType::Optional(_) => false,
                    }) {
                        Some(property_entry.name.as_str())
                    } else {
                        None
                    }
                }
                None => None,
            })
            .collect()
    }
}
