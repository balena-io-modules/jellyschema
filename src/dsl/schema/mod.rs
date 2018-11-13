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

/// A first-class collection representing a list of `PropertyEntries`, has convenience methods exposed
#[derive(Clone, Debug)]
pub struct PropertyList {
    entries: Vec<NamedProperty>,
}

/// A named property, an entry in `PropertyList`
// fixme move serialize implementation to the `output` module
#[derive(Clone, Debug, Serialize)]
pub struct NamedProperty {
    pub name: String,
    #[serde(flatten)]
    pub property: Property,
}

/// Everything that a schema at any level can represent, see schema and subschema in the spec
#[derive(Clone, Debug)]
pub struct Property {
    pub types: Option<Vec<ObjectType>>,
    pub annotations: Annotations,
    pub children: Option<PropertyList>,
    pub mapping: Option<serde_yaml::Mapping>, // TODO: real mapping support
}

/// Represents [`SchemaAnnotations`](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#schema-annotations) from the spec minus the `default` keyword, that lives in the object bounds information
#[derive(Clone, Default, Debug, Deserialize)]
pub struct Annotations {
    pub title: Option<String>,
    pub help: Option<String>,
    pub warning: Option<String>,
    pub description: Option<String>,
}

// TODO: optimization: make the methods memoize the computed result
impl PropertyList {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &Vec<NamedProperty> {
        &self.entries
    }

    /// Names of all properties
    pub fn property_names(&self) -> Vec<&str> {
        self.entries
            .iter()
            .map(|property_entry| property_entry.name.as_str())
            .collect()
    }

    /// Names of required properties only
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
