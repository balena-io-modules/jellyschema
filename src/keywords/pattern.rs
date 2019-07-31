use regex::Regex;
use serde_json::Value;

use super::{CompilationResult, Compiler, WalkContextExt};
use crate::validators;
use crate::{Scope, WalkContext};

pub const KEYWORD: &str = "pattern";

pub struct Pattern;

impl Compiler for Pattern {
    fn compile(&self, schema: &Value, ctx: &WalkContext, _: &Scope) -> CompilationResult {
        let value = compiler_non_strict_get!(schema, KEYWORD);

        if let Ok(pattern) = value.as_str().ok_or(()).and_then(|x| Regex::new(x).map_err(|_| ())) {
            return Ok(Some(Box::new(validators::pattern::Pattern { pattern })));
        }

        ctx.compilation_error(KEYWORD, "expected regular expression")
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

        assert!(Pattern.compile(&json!({}), &ctx, &scope).is_ok());
    }

    #[test]
    fn valid_regex_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Pattern.compile(&json!({KEYWORD: "^[0-9]+$"}), &ctx, &scope).is_ok());
    }

    #[test]
    fn invalid_regex_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Pattern.compile(&json!({KEYWORD: "^[0-9"}), &ctx, &scope).is_err());
    }

    #[test]
    fn other_types_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Pattern.compile(&json!({KEYWORD: true}), &ctx, &scope).is_err());
        assert!(Pattern.compile(&json!({KEYWORD: 10}), &ctx, &scope).is_err());
        assert!(Pattern.compile(&json!({KEYWORD: 10.2}), &ctx, &scope).is_err());
        assert!(Pattern.compile(&json!({KEYWORD: {}}), &ctx, &scope).is_err());
        assert!(Pattern.compile(&json!({KEYWORD: []}), &ctx, &scope).is_err());
    }
}
