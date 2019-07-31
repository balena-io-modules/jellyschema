use serde_json::Value;

use crate::validators::{ValidationState, WalkContextExt};
use crate::WalkContext;

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    let data = validator_non_strict_as!(data.as_str());

    match data.parse::<std::net::Ipv6Addr>() {
        Ok(_) => ValidationState::new(),
        Err(_) => ctx.validation_error("type", "expected valid IPv6 address").into(),
    }
}

data_type!("ipv6-address", validator: validate);
