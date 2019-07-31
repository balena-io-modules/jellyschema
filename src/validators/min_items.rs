use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::WalkContext;

pub struct MinItems {
    pub number: usize,
}

impl Validator for MinItems {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let array = validator_non_strict_as!(data.as_array());

        if array.len() < self.number {
            ctx.validation_error(
                "minItems",
                format!("array length must be greater or equal to '{}'", self.number),
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
        let min = MinItems { number: 2 };
        let value = json!([1, 2]);
        assert!(min.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn array_length_is_greater() {
        let min = MinItems { number: 2 };
        let value = json!([1, 2, 3]);
        assert!(min.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn array_length_is_lower() {
        let min = MinItems { number: 5 };
        let value = json!([1, 2, 3, 4]);
        assert!(!min.validate(&value, &WalkContext::new()).is_valid());
    }
}
