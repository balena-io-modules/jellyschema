use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::validators::{ValidationState, WalkContextExt};
use crate::WalkContext;

lazy_static! {
    // data:text/plain;name=test.txt;base64,aGV...
    static ref FILE_REGEX: Regex =
        Regex::new(r"^data:.*;name=(.*);([a-zA-Z0-9]+),(.*)$").unwrap();
}

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    let data = validator_non_strict_as!(data.as_str());

    let captures = match FILE_REGEX.captures(data) {
        Some(x) => x,
        _ => return ctx.validation_error("type", "expected `file`").into(),
    };

    if (&captures[1]).is_empty() {
        return ctx.validation_error("type", "expected file name").into();
    }

    if &captures[2] != "base64" {
        return ctx.validation_error("type", "expected base64 encoding").into();
    }

    match base64::decode(&captures[3]) {
        Ok(_) => ValidationState::new(),
        Err(_) => ctx.validation_error("type", "unable to decode base64").into(),
    }
}

data_type!("file", validator: validate);
