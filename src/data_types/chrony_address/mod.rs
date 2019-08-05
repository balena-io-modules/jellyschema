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

fn validate_as_hostname(data: &str, ctx: &WalkContext) -> ValidationState {
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

fn validate_as_ip_address(data: &str, ctx: &WalkContext) -> ValidationState {
    match data.parse::<std::net::IpAddr>() {
        Ok(_) => ValidationState::new(),
        Err(_) => ctx.validation_error("type", "invalid chrony-address").into(),
    }
}

// https://github.com/balena-os/meta-balena/blob/v2.29.2/meta-resin-common/recipes-connectivity/resin-ntp-config/resin-ntp-config/resin-ntp-config#L19
//
//  add server address [option]...
//
//     The add server command allows a new NTP server to be added whilst chronyd is running.
//     Following the words add server, the syntax of the following parameters and options is similar to that
//     for the server directive in the configuration file. The following server options can be set in the
//     command: port, minpoll, maxpoll, presend, maxdelayratio, maxdelay, key.
//     An example of using this command is shown below:
//     add server foo.example.net minpoll 6 maxpoll 10 key 25
//
// Additional options not supported, validate either as ipv4, ipv6 or hostname
fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    let data = validator_non_strict_as!(data.as_str());

    let state = validate_as_hostname(data, ctx);
    if state.is_valid() {
        return state;
    }

    validate_as_ip_address(data, ctx)
}

data_type!("chrony-address", validator: validate);
