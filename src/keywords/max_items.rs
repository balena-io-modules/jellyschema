use serde_json::Value;

use super::{CompilationResult, Compiler, WalkContextExt};
use crate::validators;
use crate::{Scope, WalkContext};

pub const KEYWORD: &str = "maxItems";

pub struct MaxItems;

impl Compiler for MaxItems {
    fn compile(&self, schema: &Value, ctx: &WalkContext, _: &Scope) -> CompilationResult {
        let value = compiler_non_strict_get!(schema, KEYWORD);

        if let Some(number) = value.as_u64() {
            return Ok(Some(Box::new(validators::max_items::MaxItems {
                number: number as usize,
            })));
        }

        ctx.compilation_error(KEYWORD, "expected number")
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

        assert!(MaxItems.compile(&json!({}), &ctx, &scope).is_ok());
    }

    #[test]
    fn integer_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(MaxItems.compile(&json!({KEYWORD: 10}), &ctx, &scope).is_ok());
    }

    #[test]
    fn negative_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(MaxItems.compile(&json!({KEYWORD: -10}), &ctx, &scope).is_err());
    }

    #[test]
    fn other_types_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(MaxItems.compile(&json!({KEYWORD: true}), &ctx, &scope).is_err());
        assert!(MaxItems.compile(&json!({KEYWORD: "foo"}), &ctx, &scope).is_err());
        assert!(MaxItems.compile(&json!({KEYWORD: 10.2}), &ctx, &scope).is_err());
        assert!(MaxItems.compile(&json!({KEYWORD: {}}), &ctx, &scope).is_err());
        assert!(MaxItems.compile(&json!({KEYWORD: []}), &ctx, &scope).is_err());
    }
}