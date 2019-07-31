use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::WalkContext;

pub struct Enum {
    pub items: Vec<Value>,
}

impl Validator for Enum {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let mut equals_count = 0;

        for item in &self.items {
            if item == data {
                equals_count += 1;
            }
            if equals_count > 1 {
                break;
            }
        }

        if equals_count == 1 {
            ValidationState::new()
        } else {
            ctx.validation_error("enum", "'{}' does not equal to any of the enum items")
                .into()
        }
    }
}
