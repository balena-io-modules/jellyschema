use serde_json::Value;

use super::{CompilationResult, Compiler, WalkContextExt};
use crate::path::Path;
use crate::validators;
use crate::{Scope, WalkContext};

pub const KEYWORD: &str = "uniqueItems";

pub struct UniqueItems;

impl Compiler for UniqueItems {
    fn compile(&self, schema: &Value, ctx: &WalkContext, _: &Scope) -> CompilationResult {
        let value = compiler_non_strict_get!(schema, KEYWORD);

        match value {
            Value::Bool(unique) => Ok(Some(Box::new(validators::unique_items::UniqueItems {
                unique: *unique,
                paths: None,
            }))),

            Value::Array(string_paths) => {
                let mut paths: Vec<Path> = vec![];

                for (idx, string_path) in string_paths.iter().enumerate() {
                    let ctx = ctx.push(idx);

                    if !string_path.is_string() {
                        return ctx.compilation_error(KEYWORD, "expected string");
                    }

                    match string_path.as_str().unwrap().parse() {
                        Ok(path) => paths.push(path),
                        Err(_) => return ctx.compilation_error(KEYWORD, "expected JSON path"),
                    }
                }

                if paths.is_empty() {
                    return ctx.compilation_error(KEYWORD, "expected at least one JSON path");
                }

                Ok(Some(Box::new(validators::unique_items::UniqueItems {
                    unique: false,
                    paths: Some(paths),
                })))
            }

            _ => ctx.compilation_error(KEYWORD, "expected boolean or an array of JSON paths"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn missing_keyword_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(UniqueItems.compile(&json!({}), &ctx, &scope).is_ok());
    }

    #[test]
    fn boolean_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(UniqueItems.compile(&json!({KEYWORD: true}), &ctx, &scope).is_ok());
        assert!(UniqueItems.compile(&json!({KEYWORD: false}), &ctx, &scope).is_ok());
    }

    #[test]
    fn empty_array_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(UniqueItems.compile(&json!({KEYWORD: []}), &ctx, &scope).is_err());
    }

    #[test]
    fn invalid_paths_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(UniqueItems
            .compile(&json!({KEYWORD: ["$['foo"]}), &ctx, &scope)
            .is_err());
    }

    #[test]
    fn one_path_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(UniqueItems
            .compile(&json!({KEYWORD: ["$['foo']"]}), &ctx, &scope)
            .is_ok());
    }

    #[test]
    fn multiple_paths_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(UniqueItems
            .compile(&json!({KEYWORD: ["$['foo']", "$['bar'][0]"]}), &ctx, &scope)
            .is_ok());
    }

    #[test]
    fn dotted_notation_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(UniqueItems
            .compile(&json!({KEYWORD: ["$.foo.2"]}), &ctx, &scope)
            .is_ok());
    }

    #[test]
    fn other_types_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(UniqueItems.compile(&json!({KEYWORD: 10}), &ctx, &scope).is_err());
        assert!(UniqueItems.compile(&json!({KEYWORD: 10.2}), &ctx, &scope).is_err());
        assert!(UniqueItems.compile(&json!({KEYWORD: "foo"}), &ctx, &scope).is_err());
        assert!(UniqueItems.compile(&json!({KEYWORD: {}}), &ctx, &scope).is_err());
    }
}
