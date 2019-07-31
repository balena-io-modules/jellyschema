use serde_json::Value;

use super::{CompilationResult, Compiler, WalkContextExt};
use crate::validators;
use crate::{Scope, WalkContext};

pub const KEYWORD: &str = "enum";

pub struct Enum;

impl Compiler for Enum {
    fn compile(&self, schema: &Value, ctx: &WalkContext, _: &Scope) -> CompilationResult {
        let value = compiler_non_strict_get!(schema, KEYWORD);

        if let Some(values) = value.as_array() {
            let ctx = ctx.push(KEYWORD);

            let mut items = vec![];

            for (idx, value) in values.iter().enumerate() {
                let ctx = ctx.push(idx);

                if let Some(object) = value.as_object() {
                    if let Some(value) = object.get("value") {
                        items.push(value.clone())
                    } else {
                        return ctx.compilation_error("value", "missing keyword");
                    }
                } else {
                    items.push(value.clone());
                }
            }

            if items.is_empty() {
                return ctx.compilation_error(KEYWORD, "must contain at least one item");
            }

            Ok(Some(Box::new(validators::enum_::Enum { items })))
        } else {
            return ctx.compilation_error(KEYWORD, "array expected");
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

        assert!(Enum.compile(&json!({}), &ctx, &scope).is_ok());
    }

    #[test]
    fn empty_array_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Enum.compile(&json!({KEYWORD: []}), &ctx, &scope).is_err());
    }

    #[test]
    fn other_types_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Enum.compile(&json!({KEYWORD: 10}), &ctx, &scope).is_err());
        assert!(Enum.compile(&json!({KEYWORD: 10.2}), &ctx, &scope).is_err());
        assert!(Enum.compile(&json!({KEYWORD: true}), &ctx, &scope).is_err());
        assert!(Enum.compile(&json!({KEYWORD: {}}), &ctx, &scope).is_err());
        assert!(Enum.compile(&json!({KEYWORD: "foo"}), &ctx, &scope).is_err());
    }
}
