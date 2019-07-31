use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::validators::{ValidationState, WalkContextExt};
use crate::WalkContext;

lazy_static! {
    // ajv v6.7.0 compatible
    // https://github.com/epoberezkin/ajv/blob/v6.7.0/lib/compile/formats.js
    static ref HOSTNAME_REGEX: Regex =
        Regex::new(r"^(?i)[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[-0-9a-z]{0,61}[0-9a-z])?)*$").unwrap();
}

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    let data = validator_non_strict_as!(data.as_str());

    if !HOSTNAME_REGEX.is_match(data) {
        return ctx.validation_error("type", "doesn't match regular expression").into();
    }

    let len = data.chars().count();

    // TODO Check specification, isn't this limit for ASCII, not UTF-8?
    if len > 255 {
        return ctx
            .validation_error("type", "maximum hostname length is 255 characters")
            .into();
    }

    ValidationState::new()
}

data_type!("hostname", validator: validate);
