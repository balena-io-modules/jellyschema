use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::WalkContext;

pub struct MaxLength {
    pub number: usize,
}

impl Validator for MaxLength {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let s = validator_non_strict_as!(data.as_str());
        let len = s.chars().count();

        if self.number < len {
            ctx.validation_error(
                "maxLength",
                format!("'{}' string length must be lower or equal to '{}'", s, self.number),
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
    fn string_length_equals_to_max_length() {
        let ml = MaxLength { number: 10 };
        let value = json!("0123456789");
        assert!(ml.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn string_length_is_greater_than_max_length() {
        let ml = MaxLength { number: 5 };
        let value = json!("0123456789");
        assert!(!ml.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn string_length_is_lower_than_max_length() {
        let ml = MaxLength { number: 15 };
        let value = json!("0123456789");
        assert!(ml.validate(&value, &WalkContext::new()).is_valid());
    }
}
