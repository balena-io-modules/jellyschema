use serde_json::Value;

use super::{BoxedValidator, ValidationState, Validator, WalkContextExt};
use crate::{Schema, WalkContext};

pub struct Type {
    pub name: String,
    pub optional: bool,
    pub custom_schema: Option<Schema>,
    pub custom_validator: Option<BoxedValidator>,
}

pub(crate) fn is_builtin_type(name: &str) -> bool {
    match name {
        "array" | "object" | "boolean" | "integer" | "number" | "string" => true,
        _ => false,
    }
}

impl Type {
    fn validate_builtin_type(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let valid = match self.name.as_str() {
            "array" => data.is_array(),
            "object" => data.is_object(),
            "boolean" => data.is_boolean(),
            "integer" => data.is_u64() || data.is_i64(),
            "number" => data.is_number(),
            "string" => data.is_string(),
            _ => unreachable!("Must be synced with is_builtin() fn"),
        };

        if valid {
            ValidationState::new()
        } else {
            ctx.validation_error("type", format!("'{}' expected", self.name)).into()
        }
    }
}

impl Validator for Type {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        if data.is_null() {
            if self.optional {
                return ValidationState::new();
            } else {
                return ctx.validation_error("type", format!("'{}' expected", self.name)).into();
            }
        }

        if is_builtin_type(self.name.as_str()) {
            return self.validate_builtin_type(data, ctx);
        }

        if let Some(custom_schema) = &self.custom_schema {
            let state = custom_schema.validate(data);

            if state.is_invalid() {
                return state;
            }
        }

        if let Some(custom_validator) = &self.custom_validator {
            return custom_validator.validate(data, ctx);
        }

        ValidationState::new()
    }
}
