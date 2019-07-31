use float_cmp::ApproxEq;
use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::WalkContext;

pub struct MultipleOf {
    pub number: f64,
}

impl Validator for MultipleOf {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let number = validator_non_strict_as!(data.as_f64());

        let remainder = number % self.number;
        if 0f64.approx_eq(remainder, (2.0 * std::f64::EPSILON, 2)) {
            ValidationState::new()
        } else {
            ctx.validation_error(
                "multipleOf",
                format!("'{}' is not multiple of '{}'", number, self.number),
            )
            .into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn value_is_multiple_of() {
        let mof = MultipleOf { number: 1.0 };
        let value = json!(10);
        assert!(mof.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn value_is_not_multiple_of() {
        let mof = MultipleOf { number: 1.0 };
        let value = json!(12.1);
        assert!(!mof.validate(&value, &WalkContext::new()).is_valid());
    }
}
