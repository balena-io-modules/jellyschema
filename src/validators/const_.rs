use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::WalkContext;

pub struct Const {
    pub item: Value,
}

impl Validator for Const {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        if self.item != *data {
            ctx.validation_error("const", format!("'{}' does not equal to '{}'", data, self.item))
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
    fn integer_equals() {
        let c = Const { item: json!(10) };
        let value = json!(10);
        assert!(c.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn string_equals() {
        let c = Const { item: json!("foo") };
        let value = json!("foo");
        assert!(c.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn bool_equals() {
        let c = Const { item: json!(true) };
        let value = json!(true);
        assert!(c.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn bool_does_not_equals() {
        let c = Const { item: json!(true) };
        let value = json!(false);
        assert!(!c.validate(&value, &WalkContext::new()).is_valid());
    }
}
