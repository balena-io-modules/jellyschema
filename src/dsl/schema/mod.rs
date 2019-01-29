//! Top-level constructs representing the configuration DSL language
use std::collections::HashMap;

use balena_temen::ast::Expression;
use regex::Regex;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::dsl::schema::object_types::ObjectType;
use crate::dsl::schema::when::DependencyGraph;

pub mod deserialization;
pub mod compiler;
pub mod object_types;
pub mod when;
mod dynamic;

/// Represents the root of the yaml DSL document
#[derive(Clone, Debug)]
pub struct DocumentRoot {
    pub version: u64,
    pub schema: Option<Schema>,
    /// the whole dependency tree for all the subschemas. Recursively. Used for `when` conditions
    pub dependencies: Option<DependencyGraph>,
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
    pub object_type: ObjectType,
    pub annotations: Annotations,
    /// children of a schema are all schemas defined inside of this schema
    pub children: Option<SchemaList>,
    /// this represents `keys` and `values` - defining dynamic objects
    pub dynamic: Option<Box<KeysValues>>,
    /// this is th DSL mapping, to and from output formats (e.g. config files etc)
    pub mapping: Option<serde_yaml::Mapping>,
    // TODO: real mapping support
    pub when: Option<Expression>,
    /// unparsed formula, can't be evaluated by CDSL as we don't have data, just schema
    pub formula: Option<String>,
}

/// Represents [`SchemaAnnotations`](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#schema-annotations) from the spec minus the `default` keyword, that lives in the object bounds information
#[derive(Clone, Default, Debug, Deserialize)]
pub struct Annotations {
    pub title: Option<String>,
    pub help: Option<String>,
    pub warning: Option<String>,
    pub description: Option<String>,
    pub widget: Option<Widget>,
    pub orderable: Option<bool>,
    pub addable: Option<bool>,
    pub removable: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Widget {
    Textarea,
}

#[derive(Clone, Debug)]
pub struct KeysSchema {
    pub pattern: Regex,
    pub title: Option<String>,
}

#[derive(Clone, Debug)]
pub struct KeysValues {
    pub keys: KeysSchema,
    pub values: Schema,
}

impl NamedSchema {
    /// Unpacks named schema into (name, schema)
    pub fn unpack(&self) -> (&str, &Schema) {
        (self.name.as_ref(), &self.schema)
    }
}

// TODO: optimization: make the methods memoize the computed result
impl SchemaList {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// schema name -> Schema
    pub fn all_as_map(&self) -> HashMap<&str, &Schema> {
        self.entries().iter().map(|schema| schema.unpack()).collect()
    }

    pub fn entries(&self) -> &Vec<NamedSchema> {
        &self.entries
    }

    pub fn dependent_schemas(&self) -> Vec<&NamedSchema> {
        self.entries
            .iter()
            .filter(|named_schema| named_schema.schema.when.is_some()) // TODO: see if this is enough
            .collect()
    }

    pub fn independent_schemas(&self) -> Vec<&NamedSchema> {
        self.entries
            .iter()
            .filter(|named_schema| named_schema.schema.when.is_none()) // TODO: see if this is enough
            .collect()
    }

    /// schema name -> Schema
    pub fn independent_as_map(&self) -> HashMap<&str, &Schema> {
        self.independent_schemas()
            .iter()
            .map(|schema| schema.unpack())
            .collect()
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
            .filter_map(|named_schema| {
                let object_type = &named_schema.schema.object_type;
                match object_type {
                    ObjectType::Required(_) => Some(named_schema.name.as_str()),
                    ObjectType::Optional(_) => None,
                }
            })
            .collect()
    }
}

impl Annotations {
    pub fn with_title_option(title: Option<String>) -> Annotations {
        let default = Annotations::default();
        Annotations { title, ..default }
    }
}
