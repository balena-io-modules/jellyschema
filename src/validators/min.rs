use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::WalkContext;

pub struct Min {
    pub number: f64,
}

impl Validator for Min {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let number = validator_non_strict_as!(data.as_f64());

        if number < self.number {
            ctx.validation_error(
                "min",
                format!("'{}' must be greater or equal to '{}'", number, self.number),
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
    fn value_equals() {
        let min = Min { number: 10.0 };
        let value = json!(10);
        assert!(min.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn value_is_greater() {
        let min = Min { number: 10.0 };
        let value = json!(11);
        assert!(min.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn value_is_lower() {
        let min = Min { number: 10.0 };
        let value = json!(9);
        assert!(!min.validate(&value, &WalkContext::new()).is_valid());
    }
}
