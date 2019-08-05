use chrono;
use serde_json::Value;

use crate::validators::{ValidationState, WalkContextExt};
use crate::WalkContext;

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    let data = validator_non_strict_as!(data.as_str());

    if chrono::DateTime::parse_from_rfc3339(data).is_err() {
        ctx.validation_error("type", "unable to parse as `date-time`").into()
    } else {
        ValidationState::new()
    }
}

data_type!("date-time", validator: validate);
