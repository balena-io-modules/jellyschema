#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate balena_cdsl;
extern crate serde_yaml;

use balena_cdsl::output::generator::Generator;

// this is a fuzz target that makes sure that we do not crash given arbitrary data
fuzz_target!(|data: &[u8]| {
if let Ok(data) = std::str::from_utf8(data) {
    if let Ok(yaml_schema) = serde_yaml::from_str(data) {
        if let Ok(generator) = Generator::with(yaml_schema) {
            generator.generate();
        }
    }
}
});
