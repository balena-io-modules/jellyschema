use serde_json::Value;

use super::{CompilationResult, Compiler, WalkContextExt};
use crate::validators;
use crate::{Scope, WalkContext};

pub const KEYWORD: &str = "type";

pub struct Type;

impl Compiler for Type {
    fn compile(&self, schema: &Value, ctx: &WalkContext, scope: &Scope) -> CompilationResult {
        let value = match schema.get(KEYWORD) {
            Some(v) => {
                if let Some(typ) = v.as_str() {
                    typ
                } else {
                    return ctx.compilation_error(KEYWORD, "expected string");
                }
            }
            None => "object",
        };

        let (name, optional) = if value.ends_with('?') {
            (&value[..value.len() - 1], true)
        } else {
            (value, false)
        };

        if !validators::type_::is_builtin_type(name) && scope.data_types.get(name).is_none() {
            return ctx.compilation_error(KEYWORD, format!("`{}` is not a known type", name));
        }

        let (custom_schema, custom_validator) = match scope.data_types.get(name) {
            Some(custom_type) => {
                let schema = scope.compile(custom_type.schema())?;
                let validator = custom_type.validator();
                (Some(schema), validator)
            }
            None => (None, None),
        };

        Ok(Some(Box::new(validators::type_::Type {
            name: name.to_string(),
            optional,
            custom_schema,
            custom_validator,
        })))
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

        assert!(Type.compile(&json!({}), &ctx, &scope).is_ok());
    }

    #[test]
    fn unknown_type_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Type.compile(&json!({KEYWORD: "foobarbaz"}), &ctx, &scope).is_err());
    }

    #[test]
    fn required_type_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Type.compile(&json!({KEYWORD: "string"}), &ctx, &scope).is_ok());
    }

    #[test]
    fn optional_type_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Type.compile(&json!({KEYWORD: "string?"}), &ctx, &scope).is_ok());
    }

    #[test]
    fn other_types_not_allowed() {
        let scope = Scope::default();
        let ctx = WalkContext::new();

        assert!(Type.compile(&json!({KEYWORD: 10}), &ctx, &scope).is_err());
        assert!(Type.compile(&json!({KEYWORD: 10.2}), &ctx, &scope).is_err());
        assert!(Type.compile(&json!({KEYWORD: true}), &ctx, &scope).is_err());
        assert!(Type.compile(&json!({KEYWORD: {}}), &ctx, &scope).is_err());
        assert!(Type.compile(&json!({KEYWORD: []}), &ctx, &scope).is_err());
    }
}
