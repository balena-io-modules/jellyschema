use std::collections::HashMap;

use balena_temen::ast::Expression;
use balena_temen::ast::ExpressionValue;
use balena_temen::ast::Identifier;
use balena_temen::ast::IdentifierValue;

use crate::dsl::schema::compiler::CompilationError;
use crate::dsl::schema::SchemaList;

// TODO: use actual DAG for representing the whole graph of dependencies between the schemas
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    // schema name -> its dependencies
    all: HashMap<String, DependencyTree>,
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
    // TODO: see if we need `merge_with` as well
}

impl DependencyGraph {
    pub fn empty() -> DependencyGraph {
        DependencyGraph { all: HashMap::new() }
    }

    fn push(self, name: &str, depends_on: &Expression) -> DependencyGraph {
        let map = match self.all.get(name) {
            None => {
                let mut map = self.all.clone();
                match depends_on.value {
                    ExpressionValue::Identifier(ref identifiers) => {
                        map.insert(name.to_string(), DependencyTree::start_with(identifiers));
                    }
                    // TODO:
                    _ => unimplemented!("walking logical expression that is not just one identifier"),
                }

                map
            }
            Some(_previous) => {
                // TODO:
                unimplemented!("merging with previously seen expression")
            }
        };

        DependencyGraph { all: map }
    }
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
