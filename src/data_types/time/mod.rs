use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::validators::{ValidationState, WalkContextExt};
use crate::WalkContext;

lazy_static! {
    // ajv v6.7.0 compatible
    // https://github.com/epoberezkin/ajv/blob/v6.7.0/lib/compile/formats.js#L104
    static ref TIME_REGEX: Regex =
        Regex::new(r"^(\d\d):(\d\d):(\d\d)(\.\d+)?(z|[+-]\d\d:\d\d)?$").unwrap();
}

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    let data = validator_non_strict_as!(data.as_str());

    let captures = match TIME_REGEX.captures(data) {
        Some(x) => x,
        _ => return ctx.validation_error("type", "expected `time`").into(),
    };

    let hour: usize = (&captures[1]).parse().expect("invalid regex");
    let min: usize = (&captures[2]).parse().expect("invalid regex");
    let sec: usize = (&captures[3]).parse().expect("invalid regex");

    if (hour <= 23 && min <= 59 && sec <= 59) || (hour == 23 && min == 59 && sec == 60) {
        ValidationState::new()
    } else {
        ctx.validation_error("type", "invalid `time` range").into()
    }
}

data_type!("time", validator: validate);
