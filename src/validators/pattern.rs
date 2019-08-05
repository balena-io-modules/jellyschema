use regex::Regex;
use serde_json::Value;

use super::{ValidationState, Validator, WalkContextExt};
use crate::WalkContext;

pub struct Pattern {
    pub pattern: Regex,
}

impl Validator for Pattern {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let s = validator_non_strict_as!(data.as_str());

        if self.pattern.is_match(s) {
            ValidationState::new()
        } else {
            ctx.validation_error("pattern", format!("'{}' does not match '{}' pattern", s, self.pattern))
                .into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn value_matches_pattern() {
        let pat = Pattern {
            pattern: Regex::new("^[0-9]+$").unwrap(),
        };
        let value = json!("10");
        assert!(pat.validate(&value, &WalkContext::new()).is_valid());
    }

    #[test]
    fn value_does_not_match_pattern() {
        let pat = Pattern {
            pattern: Regex::new("^[0-9]+$").unwrap(),
        };
        let value = json!("1a0");
        assert!(!pat.validate(&value, &WalkContext::new()).is_valid());
    }
}
