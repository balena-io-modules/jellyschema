//! Top-level constructs representing the configuration DSL language
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::dsl::schema::object_types::ObjectType;

pub mod deserialization;
pub mod compiler;
pub mod object_types;

/// Represents the root of the yaml DSL document
#[derive(Clone, Debug)]
pub struct DocumentRoot {
    pub version: u64,
    pub schema: Option<Schema>,
}

/// A first-class collection of `NamedSchema`'s, has convenience methods exposed
#[derive(Clone, Debug)]
pub struct NamedSchemaList {
    entries: Vec<NamedSchema>,
}

/// A schema with an associated name, an entry in `SchemaList`
// fixme move serialize implementation to the `output` module
#[derive(Clone, Debug, Serialize)]
pub struct NamedSchema {
    pub name: String,
    #[serde(flatten)]
    pub schema: Schema,
}

/// Everything that a schema at any level can represent, see schema and subschema in the spec
#[derive(Clone, Debug)]
pub struct Schema {
    pub types: Option<Vec<ObjectType>>,
    pub annotations: Annotations,
    pub children: Option<NamedSchemaList>,
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
impl NamedSchemaList {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &Vec<NamedSchema> {
        &self.entries
    }

    pub fn all_schema_names(&self) -> Vec<&str> {
        self.entries
            .iter()
            .map(|named_schema| named_schema.name.as_str())
            .collect()
    }

    pub fn required_schema_names(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter_map(|named_schema| match &named_schema.schema.types {
                Some(type_list) => {
                    if type_list.iter().any(|type_spec| match type_spec {
                        ObjectType::Required(_) => true,
                        ObjectType::Optional(_) => false,
                    }) {
                        Some(named_schema.name.as_str())
                    } else {
                        None
                    }
                }
                None => None,
            })
            .collect()
    }
}
