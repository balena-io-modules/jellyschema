use serde_json::Value;

use super::{CompilationResult, Compiler, WalkContextExt};
use crate::{Scope, WalkContext};

pub const KEYWORD: &str = "version";

pub struct Version;

impl Compiler for Version {
    fn compile(&self, schema: &Value, ctx: &WalkContext, _: &Scope) -> CompilationResult {
        let value = compiler_non_strict_get!(schema, KEYWORD);

        if let Some(version) = value.as_u64() {
            if version != 1 {
                return ctx.compilation_error(KEYWORD, "version `1` supported only");
            }

            return Ok(None);
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

        assert!(Version.compile(&json!({}), &ctx, &scope).is_ok());
    }

    #[test]
    fn version_one_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Version.compile(&json!({KEYWORD: 1}), &ctx, &scope).is_ok());
    }

    #[test]
    fn other_versions_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Version.compile(&json!({KEYWORD: 0}), &ctx, &scope).is_err());
        assert!(Version.compile(&json!({KEYWORD: 2}), &ctx, &scope).is_err());
        assert!(Version.compile(&json!({KEYWORD: 3}), &ctx, &scope).is_err());
    }

    #[test]
    fn other_types_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Version.compile(&json!({KEYWORD: true}), &ctx, &scope).is_err());
        assert!(Version.compile(&json!({KEYWORD: "foo"}), &ctx, &scope).is_err());
        assert!(Version.compile(&json!({KEYWORD: 10.2}), &ctx, &scope).is_err());
        assert!(Version.compile(&json!({KEYWORD: {}}), &ctx, &scope).is_err());
        assert!(Version.compile(&json!({KEYWORD: []}), &ctx, &scope).is_err());
    }
}
