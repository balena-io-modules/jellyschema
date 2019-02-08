#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate jellyschema;
extern crate serde_yaml;

use std::str::FromStr;
use jellyschema::{
    schema::Schema,
    generator::generate_json_ui_schema
};

// this is a fuzz target that makes sure that we do not crash given arbitrary data
fuzz_target!(|data: &[u8]| {
    if let Ok(data) = std::str::from_utf8(data) {
        if let Ok(schema) = Schema::from_str(data) {
            let _ = generate_json_ui_schema(&schema);
        }
    }
});
