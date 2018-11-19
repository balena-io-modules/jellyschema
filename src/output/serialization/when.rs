use std::collections::HashMap;

use serde::ser::Error;
use serde::ser::SerializeMap;
use serde::Serialize;
use serde_json::Value;

use crate::dsl::schema::deserialization::DependencyGraph;
use crate::dsl::schema::NamedSchema;
use crate::dsl::schema::SchemaList;

pub fn serialize_schema_list_dependencies<O, E, S>(
    schema_list: &SchemaList,
    dependencies: &DependencyGraph,
    map: &mut S,
) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    let mut dependencies_map = HashMap::new();
    for schema in schema_list.dependent_schemas() {
        // FIXME: encode this rule into the typesystem so it becomes impossible to represent such state
        if !dependencies.contains(&schema.name) {
            panic!("internal data inconsistency between schema list and dependency list");
        }
        let schema_dependencies = dependencies.dependencies_for(&schema.name);

        for dependency_name in schema_dependencies {
            let mut possibilities = vec![];
            possibilities.push(false_branch(dependency_name)?);
            possibilities.push(true_branch(&schema, dependency_name)?);

            let mut one_of_wrapper = HashMap::new();
            one_of_wrapper.insert("oneOf", possibilities);
            dependencies_map.insert(dependency_name, one_of_wrapper);
        }
    }

    if !dependencies_map.is_empty() {
        map.serialize_entry("dependencies", &dependencies_map)?;
    }
    Ok(())
}

fn false_branch<E>(dependency_name: &str) -> Result<HashMap<&str, Value>, E>
where
    E: Error,
{
    let mut all_variables_on_false_branch: HashMap<&str, Value> = HashMap::new();
    let mut false_branch: HashMap<&str, Value> = HashMap::new();
    let mut false_value = HashMap::new();
    false_value.insert("enum", vec![false]);
    false_branch.insert(dependency_name, to_value(&false_value)?);
    all_variables_on_false_branch.insert("properties", to_value(&false_branch)?);
    let mut required = vec![];
    required.push(dependency_name);
    all_variables_on_false_branch.insert("required", to_value(&required)?);
    let mut order = vec![];
    order.push(dependency_name);
    all_variables_on_false_branch.insert("$$order", to_value(&order)?);
    Ok(all_variables_on_false_branch)
}

fn true_branch<'a, E>(schema: &NamedSchema, dependency_name: &'a str) -> Result<HashMap<&'a str, Value>, E>
where
    E: Error,
{
    let mut all_variables_on_true_branch: HashMap<&str, Value> = HashMap::new();
    let mut true_branch: HashMap<&str, Value> = HashMap::new();
    let mut true_value = HashMap::new();
    true_value.insert("enum", vec![true]);
    true_branch.insert(dependency_name, to_value(&true_value)?);
    true_branch.insert(&schema.name, to_value(&schema.schema)?);
    all_variables_on_true_branch.insert("properties", to_value(&true_branch)?);
    let mut required = vec![];
    required.push(dependency_name);
    required.push(&schema.name);
    all_variables_on_true_branch.insert("required", to_value(&required)?);
    let mut order = vec![];
    order.push(dependency_name);
    order.push(&schema.name);
    all_variables_on_true_branch.insert("$$order", to_value(&order)?);
    Ok(all_variables_on_true_branch)
}

fn to_value<T, E>(value: &T) -> Result<Value, E>
where
    T: Serialize,
    E: Error,
{
    serde_json::to_value(value).map_err(|e| Error::custom(format!("{:#?}", e)))
}
