//! Top-level constructs representing the configuration DSL language
use balena_temen::ast::Expression;
use ego_tree::Tree;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::dsl::schema::object_types::ObjectType;
use crate::dsl::schema::deserialization::DependencyForest;

pub mod deserialization;
pub mod compiler;
pub mod object_types;

/// Represents the root of the yaml DSL document
#[derive(Clone, Debug)]
pub struct DocumentRoot {
    pub version: u64,
    pub schema: Option<Schema>,
    /// the whole dependency tree for all the subschemas. recursively
    pub dependencies: Option<DependencyForest>,
}

/// A first-class collection of `NamedSchema`'s, has convenience methods exposed
#[derive(Clone, Debug)]
pub struct SchemaList {
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
    /// children of a schema are all schemas defined inside of this schema
    pub children: Option<SchemaList>,
    /// this is th DSL mapping, to and from output formats (e.g. config files etc)
    pub mapping: Option<serde_yaml::Mapping>, // TODO: real mapping support
    pub when: Option<Expression>,
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
impl SchemaList {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &Vec<NamedSchema> {
        &self.entries
    }

    /// names of all schemas that do not depend on any other schema
    pub fn independent_schema_names(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|named_schema| named_schema.schema.when.is_none()) // TODO: see if this is enough
            .map(|named_schema| named_schema.name.as_str())
            .collect()
    }

    pub fn required_schema_names(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|named_schema| named_schema.schema.when.is_none()) // TODO: see if this is enough
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
