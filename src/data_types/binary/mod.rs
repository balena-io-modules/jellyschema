use serde_json::Value;

use crate::validators::{ValidationState, WalkContextExt};
use crate::WalkContext;

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    let data = validator_non_strict_as!(data.as_str());

    match base64::decode(data) {
        Ok(_) => ValidationState::new(),
        Err(_) => ctx.validation_error("type", "unable to decode base64").into(),
    }
}

data_type!("binary", validator: validate);
