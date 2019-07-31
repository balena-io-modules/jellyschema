//! An example how to add a custom data type.
//!
//! Well known data types should be included in the Jelly Schema library directly.
//! You can use this interface to add rare types that are useful for you,
//! but not for others (they won't be used widely).
use serde_json::json;

use jellyschema::data_types::DataType;
use jellyschema::ScopeBuilder;

struct UnprivilegedPort;

// Unprivileged port data type schema
//
// - based on the `port` schema
// - narrowed range 1024-65535 (port has 0-65535)
const DATA_TYPE_SCHEMA: &str = r##"
type: port
min: 1024
"##;

impl DataType for UnprivilegedPort {
    // This is the only required method, others like validator, generator, ...
    // can be omitted if you don't need to do custom validation or generate
    // a value.
    fn schema(&self) -> &str {
        DATA_TYPE_SCHEMA
    }
}

fn main() -> Result<(), String> {
    // Create scope with default keywords, data types and add our new data type
    let scope = ScopeBuilder::default()
        .data_type("unprivileged-port", Box::new(UnprivilegedPort))
        .build();

    // Compile simple schema with our new data type
    let schema = scope
        .compile("type: unprivileged-port")
        .map_err(|e| format!("Unable to compile schema: {}", e))?;

    // Some ports to test
    let ports = [1020, 1024, 65535, 65536];

    for port in &ports {
        println!("Validating port '{}'", port);

        let data = json!(port);
        let state = schema.validate(&data);

        if state.is_valid() {
            println!(" - port '{}' is valid", port);
        } else {
            println!(" - port '{}' is invalid", port);
            for e in state.errors() {
                println!(
                    " - path: '{}', keyword: '{}', message: '{}'",
                    e.data_path, e.keyword, e.message
                );
            }
        }
    }

    Ok(())
}
