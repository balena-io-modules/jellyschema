use serde_json::Value;

use super::{CompilationResult, Compiler, WalkContextExt};
use crate::validators;
use crate::{Scope, WalkContext};

pub const KEYWORD: &str = "items";

pub struct Items;

impl Compiler for Items {
    fn compile(&self, schema: &Value, ctx: &WalkContext, scope: &Scope) -> CompilationResult {
        let value = compiler_non_strict_get!(schema, KEYWORD);

        if value.is_object() {
            let ctx = ctx.push(KEYWORD);

            let schema = scope.compile_from_value_in_context(value.clone(), &ctx)?;
            return Ok(Some(Box::new(validators::items::Items { schemas: vec![schema] })));
        }

        if value.is_array() {
            let mut schemas = vec![];

            let ctx = ctx.push(KEYWORD);
            for (idx, value) in value.as_array().unwrap().iter().enumerate() {
                let ctx = ctx.push(idx);
                let schema = scope.compile_from_value_in_context(value.clone(), &ctx)?;
                schemas.push(schema);
            }

            if schemas.is_empty() {
                return ctx.compilation_error(KEYWORD, "expected at least one schema");
            }

            return Ok(Some(Box::new(validators::items::Items { schemas })));
        }

        ctx.compilation_error(KEYWORD, "expected schema or an array of schemas")
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

        assert!(Items.compile(&json!({}), &ctx, &scope).is_ok());
    }

    #[test]
    fn object_with_valid_schema_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Items
            .compile(&json!({KEYWORD: {"type": "string"}}), &ctx, &scope)
            .is_ok());
    }

    #[test]
    fn object_with_invalid_schema_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Items
            .compile(&json!({KEYWORD: {"type": "foobarbaz"}}), &ctx, &scope)
            .is_err());
    }

    #[test]
    fn empty_array_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Items.compile(&json!({KEYWORD: []}), &ctx, &scope).is_err());
    }

    #[test]
    fn array_with_schema_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Items
            .compile(&json!({KEYWORD: [{"type": "string"}]}), &ctx, &scope)
            .is_ok());
    }

    #[test]
    fn array_with_invalid_schema_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Items
            .compile(&json!({KEYWORD: [{"type": "foobarbaz"}]}), &ctx, &scope)
            .is_err());
    }

    #[test]
    fn array_with_multiple_schemas_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Items
            .compile(
                &json!({KEYWORD: [{"type": "string"}, {"type": "integer"}]}),
                &ctx,
                &scope
            )
            .is_ok());
    }

    #[test]
    fn other_types_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Items.compile(&json!({KEYWORD: true}), &ctx, &scope).is_err());
        assert!(Items.compile(&json!({KEYWORD: "foo"}), &ctx, &scope).is_err());
        assert!(Items.compile(&json!({KEYWORD: 10}), &ctx, &scope).is_err());
        assert!(Items.compile(&json!({KEYWORD: 10.3}), &ctx, &scope).is_err());
    }
}
