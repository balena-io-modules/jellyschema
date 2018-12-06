//! This module is responsible for parsing the `when` keyword - creating a dependency tree between Schemas
use std::collections::HashMap;

use balena_temen::ast::Expression;
use balena_temen::ast::ExpressionValue;
use balena_temen::ast::Identifier;
use balena_temen::ast::IdentifierValue;

use crate::dsl::schema::compiler::CompilationError;
use crate::dsl::schema::SchemaList;

/// This represents the whole dependency graph of the whole yaml schema document
// TODO: use actual DAG for representing the whole graph of dependencies between the schemas
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    // schema name -> its dependencies
    all: HashMap<String, DependencyTree>,
}

/// This is all dependencies of a particular Schema, represented as names of other Schemas
#[derive(Debug, Clone)]
struct DependencyTree {
    tree: Vec<String>, // TODO: change into actual tree
}

pub fn dependencies_for_schema_list(
    maybe_list: Option<&SchemaList>,
    previous_tree: DependencyGraph,
) -> Result<DependencyGraph, CompilationError> {
    match maybe_list {
        None => Ok(DependencyGraph::empty()),
        Some(list) => {
            let mut tree = previous_tree;
            for schema in list.entries() {
                if let Some(when) = &schema.schema.when {
                    tree = tree.push(&schema.name, &when)?;
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

impl DependencyGraph {
    pub fn contains(&self, schema_name: &str) -> bool {
        self.all.contains_key(schema_name)
    }

    pub fn dependencies_for(&self, schema_name: &str) -> Vec<&str> {
        if self.contains(schema_name) {
            self.all[schema_name].tree.iter().map(|name| name.as_ref()).collect()
        } else {
            vec![]
        }
    }
}

impl DependencyGraph {
    pub fn empty() -> DependencyGraph {
        DependencyGraph { all: HashMap::new() }
    }

    fn push(self, name: &str, depends_on: &Expression) -> Result<DependencyGraph, CompilationError> {
        let map = match self.all.get(name) {
            None => {
                let mut map = self.all.clone();
                match depends_on.value {
                    ExpressionValue::Identifier(ref identifiers) => {
                        map.insert(name.to_string(), DependencyTree::start_with(identifiers)?);
                    }
                    // TODO:
                    _ => {
                        return Err(CompilationError::with_message(
                            "walking logical expression that is more than a single identifier",
                        ));
                    }
                }

                map
            }
            Some(_previous) => {
                // TODO:
                return Err(CompilationError::with_message(
                    "merging with previously seen expression in not supported yet",
                ));
            }
        };

        Ok(DependencyGraph { all: map })
    }
}

impl DependencyTree {
    fn start_with(identifiers: &Identifier) -> Result<DependencyTree, CompilationError> {
        let mut result = vec![];
        for identifier in &identifiers.values {
            match identifier {
                IdentifierValue::Name(name) => {
                    result.push(name.clone());
                }
                _ => return Err(CompilationError::with_message("unimplemented")),
            }
        }
        Ok(DependencyTree { tree: result })
    }
    // TODO: see if we need `merge_with` as well
}
