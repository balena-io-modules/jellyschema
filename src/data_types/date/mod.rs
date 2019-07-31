use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::validators::{ValidationState, WalkContextExt};
use crate::WalkContext;

lazy_static! {
    // ajv v6.7.0 compatible
    // https://github.com/epoberezkin/ajv/blob/v6.7.0/lib/compile/formats.js#L90
    static ref DATE_REGEX: Regex =
        Regex::new(r"^(\d\d\d\d)-(\d\d)-(\d\d)$").unwrap();
}

fn is_leap_year(year: usize) -> bool {
    // https://tools.ietf.org/html/rfc3339#appendix-C
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days(year: usize, month: usize) -> usize {
    const DAYS: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if month == 2 && is_leap_year(year) {
        29
    } else {
        DAYS[month - 1]
    }
}

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    let data = validator_non_strict_as!(data.as_str());

    let captures = match DATE_REGEX.captures(data) {
        Some(x) => x,
        _ => return ctx.validation_error("type", "expected `date`").into(),
    };

    let year: usize = (&captures[1]).parse().expect("invalid regex");
    let month: usize = (&captures[2]).parse().expect("invalid regex");
    let day: usize = (&captures[3]).parse().expect("invalid regex");

    if month >= 1 && month <= 12 && day >= 1 && day <= days(year, month) {
        ValidationState::new()
    } else {
        ctx.validation_error("type", "expected `date`").into()
    }
}

data_type!("date", validator: validate);
