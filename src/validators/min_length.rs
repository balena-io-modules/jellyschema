use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::WalkContext;

pub struct MinLength {
    pub number: usize,
}

impl Validator for MinLength {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let s = validator_non_strict_as!(data.as_str());
        let len = s.chars().count();

        if self.number > len {
            ctx.validation_error(
                "minLength",
                format!("'{}' string length must be greater or equal to '{}'", s, self.number),
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
    fn string_length_equals_to_min_length() {
        let ml = MinLength { number: 10 };
        let value = json!("0123456789");
        assert!(ml.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn string_length_is_greater_than_min_length() {
        let ml = MinLength { number: 5 };
        let value = json!("0123456789");
        assert!(ml.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn string_length_is_lower_than_min_length() {
        let ml = MinLength { number: 15 };
        let value = json!("0123456789");
        assert!(!ml.validate(&value, &WalkContext::new()).is_valid());
    }
}
