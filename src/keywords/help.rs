use serde_json::Value;

use super::{CompilationResult, Compiler, WalkContextExt};
use crate::{Scope, WalkContext};

pub const KEYWORD: &str = "help";

pub struct Help;

impl Compiler for Help {
    fn compile(&self, schema: &Value, ctx: &WalkContext, _scope: &Scope) -> CompilationResult {
        let value = compiler_non_strict_get!(schema, KEYWORD);

        if value.is_string() {
            Ok(None)
        } else {
            ctx.compilation_error(KEYWORD, "expected string")
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

        assert!(Help.compile(&json!({}), &ctx, &scope).is_ok());
    }

    #[test]
    fn string_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Help.compile(&json!({KEYWORD: "foo"}), &ctx, &scope).is_ok());
    }

    #[test]
    fn other_types_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Help.compile(&json!({KEYWORD: 10}), &ctx, &scope).is_err());
        assert!(Help.compile(&json!({KEYWORD: 10.2}), &ctx, &scope).is_err());
        assert!(Help.compile(&json!({KEYWORD: true}), &ctx, &scope).is_err());
        assert!(Help.compile(&json!({KEYWORD: {}}), &ctx, &scope).is_err());
        assert!(Help.compile(&json!({KEYWORD: []}), &ctx, &scope).is_err());
    }
}
