use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::validators::{ValidationState, WalkContextExt};
use crate::WalkContext;

lazy_static! {
    static ref EMAIL_REGEX: Regex =
        Regex::new(r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$").unwrap();
}

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    let data = validator_non_strict_as!(data.as_str());

    if !EMAIL_REGEX.is_match(data) {
        return ctx.validation_error("type", "doesn't match regular expression").into();
    }

    ValidationState::new()
}

data_type!("email", validator: validate);
