use serde_json::Value;

use super::{CompilationResult, Compiler};
use crate::validators;
use crate::{Scope, WalkContext};

pub const KEYWORD: &str = "const";

pub struct Const;

impl Compiler for Const {
    fn compile(&self, schema: &Value, _: &WalkContext, _scope: &Scope) -> CompilationResult {
        let value = compiler_non_strict_get!(schema, KEYWORD);
        Ok(Some(Box::new(validators::const_::Const { item: value.clone() })))
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

        assert!(Const.compile(&json!({}), &ctx, &scope).is_ok());
    }

    #[test]
    fn boolean_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Const.compile(&json!({KEYWORD: false}), &ctx, &scope).is_ok());
        assert!(Const.compile(&json!({KEYWORD: true}), &ctx, &scope).is_ok());
    }

    #[test]
    fn integer_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();
        assert!(Const.compile(&json!({KEYWORD: 10}), &ctx, &scope).is_ok());
    }

    #[test]
    fn float_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();
        assert!(Const.compile(&json!({KEYWORD: 10.2}), &ctx, &scope).is_ok());
    }

    #[test]
    fn string_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();
        assert!(Const.compile(&json!({KEYWORD: "foo"}), &ctx, &scope).is_ok());
    }

    #[test]
    fn array_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();
        assert!(Const.compile(&json!({KEYWORD: []}), &ctx, &scope).is_ok());
    }

    #[test]
    fn object_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();
        assert!(Const.compile(&json!({KEYWORD: {}}), &ctx, &scope).is_ok());
    }
}
