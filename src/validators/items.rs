use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::{Schema, WalkContext};

pub struct Items {
    pub schemas: Vec<Schema>,
}

impl Validator for Items {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let array = validator_non_strict_as!(data.as_array());

        for (idx, item) in array.iter().enumerate() {
            let ctx = ctx.push(idx);

            let mut valid_count = 0;
            let mut first_invalid_states = None;

            for schema in &self.schemas {
                let state = schema.validate_in_context(item, &ctx);

                if state.is_valid() {
                    valid_count += 1;
                } else if first_invalid_states.is_none() {
                    first_invalid_states = Some(state);
                }

                if valid_count > 1 {
                    return ctx.validation_error("items", "matches more than one schema").into();
                }
            }

            if valid_count != 1 {
                if let Some(state) = first_invalid_states.take() {
                    return state;
                }
            }

            if valid_count == 0 {
                return ctx.validation_error("items", "does not match any schema").into();
            }
        }

        ValidationState::new()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::Scope;

    use super::*;

    #[test]
    fn value_must_match_exactly_one_schema() {
        let scope = Scope::default();

        let items = Items {
            schemas: vec![
                scope.compile_from_value(json!({"type": "number", "min": 10})).unwrap(),
                scope.compile_from_value(json!({"type": "number", "min": 20})).unwrap(),
            ],
        };

        assert!(items.validate(&json!([11]), &WalkContext::new()).is_valid());
        assert!(items.validate(&json!([20]), &WalkContext::new()).is_invalid());
    }

    #[test]
    fn value_does_not_match_any_schema() {
        let scope = Scope::default();

        let items = Items {
            schemas: vec![
                scope.compile_from_value(json!({"type": "number", "min": 10})).unwrap(),
                scope.compile_from_value(json!({"type": "number", "min": 20})).unwrap(),
            ],
        };

        assert!(items.validate(&json!([8]), &WalkContext::new()).is_invalid());
    }
}
