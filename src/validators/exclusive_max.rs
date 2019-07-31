use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::WalkContext;

pub struct ExclusiveMax {
    pub number: f64,
}

impl Validator for ExclusiveMax {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let number = validator_non_strict_as!(data.as_f64());

        if number >= self.number {
            ctx.validation_error(
                "exclusiveMax",
                format!("'{}' must be lower than '{}'", number, self.number),
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
        let max = ExclusiveMax { number: 10.0 };
        let value = json!(10);
        assert!(!max.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn value_is_greater() {
        let max = ExclusiveMax { number: 10.0 };
        let value = json!(11);
        assert!(!max.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn value_is_lower() {
        let max = ExclusiveMax { number: 10.0 };
        let value = json!(9);
        assert!(max.validate(&value, &WalkContext::new()).is_valid());
    }
}
