use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::WalkContext;

pub struct MaxItems {
    pub number: usize,
}

impl Validator for MaxItems {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let array = validator_non_strict_as!(data.as_array());

        if array.len() > self.number {
            ctx.validation_error(
                "maxItems",
                format!("array length must be lower or equal to '{}'", self.number),
            )
            .into()
        } else {
            ValidationState::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn array_length_equals() {
        let max = MaxItems { number: 2 };
        let value = json!([1, 2]);
        assert!(max.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn array_length_is_greater() {
        let max = MaxItems { number: 2 };
        let value = json!([1, 2, 3]);
        assert!(!max.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn array_length_is_lower() {
        let max = MaxItems { number: 5 };
        let value = json!([1, 2, 3, 4]);
        assert!(max.validate(&value, &WalkContext::new()).is_valid());
    }
}
