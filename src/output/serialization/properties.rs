use std::collections::HashMap;

use serde::ser::{Error, Serialize, SerializeMap, Serializer};

use crate::dsl::schema::SchemaList;
use crate::dsl::schema::object_types::ObjectType;
use crate::dsl::schema::Schema;
use crate::output::serialization::object_types::object_type_name;
use crate::output::serialization::object_types::serialize_object_type;
use crate::dsl::schema::deserialization::DependencyForest;
use serde_yaml::Mapping;
use serde_json::Value;

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

                // generate the `oneOf` list of possibilities
                // there are only 2 possibilities for bool

                // false - just the outer variable - schema name

                // possibility [true, false]
                let mut possibilities_exploded = vec![];
                // false branch - just the false value
                let mut all_variables_on_false_branch : HashMap<&str, Value> = HashMap::new();
                let mut false_branch : HashMap<&str, Value> = HashMap::new();
                let mut false_value = HashMap::new();
                false_value.insert("enum", vec![false]);
                false_branch.insert(dependency_name, to_value( &false_value )?);
                all_variables_on_false_branch.insert("properties", to_value(&false_branch)?);
                let mut required = vec![];
                required.push(dependency_name);
                all_variables_on_false_branch.insert("required", to_value(&required)?);
                let mut order = vec![];
                order.push(dependency_name);
                all_variables_on_false_branch.insert("$$order", to_value(&order)?);
                possibilities_exploded.push(all_variables_on_false_branch);
                // true branch - true value indicator + the dependent variable
                let mut all_variables_on_true_branch :HashMap<&str, Value> = HashMap::new();
                let mut true_branch : HashMap<&str, Value> = HashMap::new();
                let mut true_value = HashMap::new();
                true_value.insert("enum", vec![true]);
                true_branch.insert(dependency_name, to_value( &true_value)?);
                true_branch.insert(&schema.name, to_value( &schema.schema )? );

                all_variables_on_true_branch.insert("properties", to_value(& true_branch)?);
                let mut required = vec![];
                required.push(dependency_name);
                required.push(&schema.name);
                all_variables_on_true_branch.insert("required", to_value(&required)?);
                let mut order = vec![];
                order.push(dependency_name);
                order.push(&schema.name);
                all_variables_on_true_branch.insert("$$order", to_value(&order)?);
                possibilities_exploded.push(all_variables_on_true_branch);

                let mut one_of_wrapper = HashMap::new();
                one_of_wrapper.insert("oneOf", possibilities_exploded);
                dependencies_map.insert(dependency_name, one_of_wrapper);
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

fn to_value<T, E>(value: &T) -> Result<Value, E>
where
      T: Serialize,
      E: Error
{
    serde_json::to_value( value ).
        map_err(|e| Error::custom(format!("{:#?}", e)))
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
