#[allow(clippy::all)]
// generated via `build.rs`, one test per directory in tests/data
include!(concat!(env!("OUT_DIR"), "/validator_data_tests.rs"));
include!(concat!(env!("OUT_DIR"), "/validator_errors_tests.rs"));
include!(concat!(env!("OUT_DIR"), "/generator_invalid_tests.rs"));
include!(concat!(env!("OUT_DIR"), "/generator_valid_tests.rs"));

// TODO: add quickcheck tests for system properties
// list:
// property should be required by default
// property should be listed in the order list in order
