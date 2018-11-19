use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

use crate::dsl::schema::compiler::CompilationError;
use crate::dsl::schema::DocumentRoot;
use crate::dsl::schema::NamedSchema;
use crate::dsl::schema::SchemaList;
use crate::dsl::schema::object_types::deserialization::deserialize_object_type;
use crate::dsl::schema::object_types::ObjectType;
use crate::dsl::schema::object_types::RawObjectType;
use crate::dsl::schema::Schema;

pub fn deserialize_root<E>(schema: &Value) -> Result<DocumentRoot, CompilationError>
where
    E: serde::de::Error,
{
    let maybe_root = schema.as_mapping();
    let version = match maybe_root {
        Some(mapping) => Ok({
            let version = mapping
                .get(&Value::from("version"))
                .ok_or_else(|| CompilationError::with_message("you must specify schema version"))?;
            version
                .as_u64()
                .ok_or_else(|| CompilationError::with_message("version must be a positive integer"))?
        }),
        None => Err(CompilationError::with_message(
            "root level schema needs to be a yaml mapping",
        )),
    }?;

    let schema = deserialize_schema::<serde_yaml::Error>(&schema)?;

    // this is recursive already, should get the whole tree for all children schemas
    let dependencies = dependencies_for_schema_list(schema.children.as_ref(), DependencyForest::empty())?;

    eprintln!("{:#?}", dependencies);

    Ok(DocumentRoot {
        version,
        schema: Some(schema),
        dependencies: Some(dependencies),
    })
}

/// structure representing the whole DAG of dependencies between the schemas
#[derive(Debug, Clone)]
pub struct DependencyForest {
    // schema name -> its dependencies
    all: HashMap<String, DependencyTree>,
}

impl DependencyForest {
    pub fn contains(&self, schema_name: &str) -> bool {
        return self.all.contains_key(schema_name);
    }

    pub fn dependencies_for(&self, schema_name: &str) -> Vec<&str> {
        if self.contains(schema_name) {
            self.all[schema_name].tree.iter().map(|name| name.as_ref()).collect()
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone)]
struct DependencyTree {
    tree: Vec<String>, // TODO: change into actual tree
}

impl DependencyTree {
    fn start_with(identifiers: &Identifier) -> DependencyTree {
        let mut result = vec![];
        for identifier in &identifiers.values {
            match identifier {
                IdentifierValue::Name(name) => {
                    result.push(name.clone());
                }
                _ => unimplemented!(),
            }
        }
        DependencyTree { tree: result }
    }

    fn merge_with(self, expression: &Identifier) -> DependencyTree {
        DependencyTree {
            tree: vec![], //FIXME actually merge
        }
    }
}

impl DependencyForest {
    fn empty() -> DependencyForest {
        DependencyForest { all: HashMap::new() }
    }

    fn push(self, name: &str, depends_on: &Expression) -> DependencyForest {
        let map = match self.all.get(name) {
            None => {
                let mut map = self.all.clone();
                match depends_on.value {
                    ExpressionValue::Identifier(ref identifiers) => {
                        map.insert(name.to_string(), DependencyTree::start_with(identifiers));
                    }
                    _ => unimplemented!(), // TODO: support walking logical expressions
                }

                map
            }
            Some(previous) => {
                let mut map = self.all.clone();
                // map.insert(name, previous.clone().merge_with(depends_on));
                map
            }
        };

        DependencyForest { all: map }
    }
}

fn dependencies_for_schema_list(
    maybe_list: Option<&SchemaList>,
    previous_tree: DependencyForest,
) -> Result<DependencyForest, CompilationError> {
    match maybe_list {
        None => Ok(DependencyForest::empty()),
        Some(list) => {
            let mut tree = previous_tree;
            for schema in list.entries() {
                if let Some(when) = &schema.schema.when {
                    tree = tree.push(&schema.name, &when);
                }

                if let Some(children) = &schema.schema.children {
                    for named_child in children.entries() {
                        tree = dependencies_for_schema_list(named_child.schema.children.as_ref(), tree)?;
                    }
                }
            }
            Ok(tree)
        }
    }
}

use balena_temen::ast::*;
use std::collections::HashMap;
use ego_tree::Tree;

pub fn deserialize_schema<E>(value: &Value) -> Result<Schema, E>
where
    E: Error,
{
    let yaml_mapping = value
        .as_mapping()
        .ok_or_else(|| Error::custom(format!("schema is not a yaml mapping - {:#?}", value)))?;
    let mut type_information = deserialize_object_type(&yaml_mapping)?;

    if type_information.is_none() {
        type_information = Some(vec![ObjectType::Required(RawObjectType::Object)]);
    }

    let annotations = serde_yaml::from_value(value.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize schema annotations - {}", e)))?;

    let properties = yaml_mapping.get(&Value::from("properties"));
    let properties = match properties {
        None => None,
        Some(properties) => match properties {
            Value::Sequence(sequence) => Some(sequence_to_schema_list(&sequence.to_vec())?),
            _ => return Err(Error::custom("`properties` is not a yaml sequence")),
        },
    };

    let mapping = yaml_mapping.get(&Value::from("mapping"));
    let mapping = match mapping {
        None => Ok(None),
        Some(mapping) => match mapping {
            Value::Mapping(mapping) => Ok(Some(mapping)),
            _ => Err(Error::custom(format!("cannot deserialize mapping {:#?}", mapping))),
        },
    }?;

    let when = yaml_mapping.get(&Value::from("when"));
    let when = match when {
        None => Ok(None),
        Some(mapping) => match mapping {
            Value::String(string) => {
                Ok(Some(string.parse().map_err(|e| {
                    Error::custom(format!("error parsing when expression: {}", e))
                })?))
            }
            _ => Err(Error::custom(format!("unknown shape of `when`: {:#?}", mapping))),
        },
    };
    Ok(Schema {
        types: type_information,
        annotations,
        children: properties,
        mapping: mapping.cloned(),
        when: when?,
    })
}

fn sequence_to_schema_list<E>(sequence: &[Value]) -> Result<SchemaList, E>
where
    E: Error,
{
    let list_of_maybe_entries = sequence.into_iter().map(|value| {
        let mapping = value
            .as_mapping()
            .ok_or_else(|| Error::custom(format!("cannot deserialize schema {:#?} as mapping", value)))?;
        Ok(mapping_to_named_schema(mapping)?)
    });

    let list: Result<Vec<_>, E> = list_of_maybe_entries.collect();
    let list = list?;

    Ok(SchemaList { entries: list })
}

fn mapping_to_named_schema<E>(mapping: &Mapping) -> Result<NamedSchema, E>
where
    E: Error,
{
    let (key, value) = mapping
        .into_iter()
        .next()
        .ok_or_else(|| Error::custom("cannot get first element of the sequence"))?;
    let key: String = serde_yaml::from_value(key.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize named schema name - {}", e)))?;
    let value = deserialize_schema(&value)?;
    Ok(NamedSchema {
        name: key,
        schema: value,
    })
}
