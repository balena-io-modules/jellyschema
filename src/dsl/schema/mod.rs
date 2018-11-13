//! Top-level constructs representing the configuration DSL language
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::dsl::schema::object_types::ObjectType;

pub mod deserialization;
pub mod compiler;
pub mod object_types;
/// Represents the root of the yaml DSL document
#[derive(Clone, Debug)]
pub struct SchemaRoot {
    pub version: u64,
    pub self_property: Option<Property>,
}

#[derive(Clone, Debug)]
pub struct PropertyList {
    property_names: Vec<String>,
    pub entries: Vec<PropertyEntry>,
}

// fixme move serialize implementation to the `output` module
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

/// Represents [`SchemaAnnotations`](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#schema-annotations) from the spec
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
