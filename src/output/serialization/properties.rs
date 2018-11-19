use std::collections::HashMap;

use serde::ser::{Error, Serialize, SerializeMap, Serializer};

use crate::dsl::schema::SchemaList;
use crate::dsl::schema::object_types::ObjectType;
use crate::dsl::schema::Schema;
use crate::output::serialization::object_types::object_type_name;
use crate::output::serialization::object_types::serialize_object_type;
use crate::dsl::schema::deserialization::DependencyForest;

pub fn serialize_schema_list<O, E, S>(
    schema_list: &SchemaList,
    dependencies: Option<&DependencyForest>,
    map: &mut S,
) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    let properties_map: HashMap<&str, &Schema> = schema_list
        .independent_schemas()
        .iter()
        .map(|schema| (schema.name.as_ref(), &schema.schema))
        .collect();
    if !properties_map.is_empty() {
        map.serialize_entry("properties", &properties_map)?;
    }

    let mut dependencies_map = HashMap::new();
    for schema in schema_list.dependent_schemas() {
        let dependencies = dependencies.expect("inconsistency between schema list and dependency list");
        if dependencies.contains(&schema.name) {
            // get this schema's dependencies and print them out first
            // i.e. inverse the tree
            let schema_dependencies = dependencies.dependencies_for(&schema.name);

            for dependency_name in schema_dependencies {
                dependencies_map.insert(dependency_name, &schema.name);
            }
        }
    }

    if !dependencies_map.is_empty() {
        map.serialize_entry("dependencies", &dependencies_map)?;
    }

    let required = &schema_list.required_schema_names();
    if !required.is_empty() {
        map.serialize_entry("required", required)?;
    }

    let names = &schema_list.independent_schema_names();
    if !names.is_empty() {
        map.serialize_entry("$$order", names)?;
    }
    Ok(())
}

// FIXME: do not use trait implementation as it is hard to track where this is being called from
impl Serialize for Schema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        // FIXME: investigate if we do not need dependencies from here
        serialize_schema(&self, None, &mut map)?;
        map.end()
    }
}

pub fn serialize_schema<O, E, S>(schema: &Schema, dependencies: Option<&DependencyForest>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    for title in &schema.annotations.title {
        map.serialize_entry("title", &title)?;
    }

    if let Some(types) = &schema.types {
        if types.len() == 1 {
            serialize_object_type(types[0].inner(), map)?;
        }
        if types.len() > 1 {
            serialize_type_array(types, map)?;
        }
    }

    if let Some(children) = &schema.children {
        serialize_schema_list(children, dependencies, map)?;
    }

    if let Some(mapping) = &schema.mapping {
        map.serialize_entry("$$mapping", mapping)?;
    }
    Ok(())
}

fn serialize_type_array<O, E, S>(types: &[ObjectType], map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if types.iter().any(|def| def.inner().has_bounds()) {
        return Err(Error::custom("cannot have type bounds when having a multi-type object"));
    }

    let type_names: Vec<_> = types.iter().map(|def| object_type_name(def.inner())).collect();
    map.serialize_entry("type", &type_names)?;
    Ok(())
}
