use std::collections::HashMap;

use serde::ser::Error;
use serde::ser::SerializeMap;
use serde::Serialize;
use serde_json::Value;

use crate::dsl::schema::when::DependencyGraph;
use crate::dsl::schema::NamedSchema;
use crate::dsl::schema::SchemaList;

/// this is one branch or arm of the `oneOf` statement in the `dependencies` section of the output JSONSchema
struct Branch<'a> {
    properties: HashMap<&'a str, Value>,
    /// order of properties (their names)
    order: Vec<&'a str>,
    /// names of required properties
    required: Vec<&'a str>,
}

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
        // probably we should make `dependencies` more tightly coupled with `schema` ?
        if !dependencies.contains(&schema.name) {
            panic!("internal data inconsistency between schema list and dependency list");
        }
        let schema_dependencies = dependencies.dependencies_for(&schema.name);

        for dependency_name in schema_dependencies {
            let mut possibilities = vec![];
            possibilities.push(json_friendly(&false_branch(dependency_name)?)?);
            possibilities.push(json_friendly(&true_branch(&schema, dependency_name)?)?);

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

fn false_branch<E>(dependency_name: &str) -> Result<Branch, E>
where
    E: Error,
{
    Ok(branch_with_value(false, dependency_name)?)
}

fn true_branch<'a, E>(schema: &'a NamedSchema, dependency_name: &'a str) -> Result<Branch<'a>, E>
where
    E: Error,
{
    let mut branch = branch_with_value(true, dependency_name)?;
    branch.properties.insert(&schema.name, to_value(&schema.schema)?);
    branch.required.push(&schema.name);
    branch.order.push(&schema.name);
    Ok(branch)
}

fn branch_with_value<E>(value: bool, dependency_name: &str) -> Result<Branch, E>
where
    E: Error,
{
    let mut properties: HashMap<&str, Value> = HashMap::new();
    let mut value_enum = HashMap::new();
    value_enum.insert("enum", vec![value]);
    properties.insert(dependency_name, to_value(&value_enum)?);

    let mut required = vec![];
    required.push(dependency_name);

    let mut order = vec![];
    order.push(dependency_name);

    Ok(Branch {
        properties,
        order,
        required,
    })
}

fn json_friendly<'a, E>(branch: &Branch) -> Result<HashMap<&'a str, Value>, E>
where
    E: Error,
{
    let mut branch_wrapper: HashMap<&str, Value> = HashMap::new();
    branch_wrapper.insert("properties", to_value(&branch.properties)?);
    branch_wrapper.insert("required", to_value(&branch.required)?);
    branch_wrapper.insert("$$order", to_value(&branch.order)?);
    Ok(branch_wrapper)
}

fn to_value<T, E>(value: &T) -> Result<Value, E>
where
    T: Serialize,
    E: Error,
{
    serde_json::to_value(value).map_err(|e| Error::custom(format!("{:#?}", e)))
}
