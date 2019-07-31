//! An example how to validate data against Jelly Schema.

use jellyschema::Scope;
use serde_json::json;

const SCHEMA: &str = r##"
type: integer
min: 1024
max: 65535
"##;

fn main() -> Result<(), String> {
    // Create scope with default keywords & data types
    let scope = Scope::default();

    // Compile schema
    let schema = scope
        .compile(SCHEMA)
        .map_err(|e| format!("Unable to compile schema: {}", e))?;

    // Validate some numbers against the compiled schema
    let numbers_to_test = [1023, 1024, 65536];

    for number in &numbers_to_test {
        println!("Validating '{}'", number);

        let data = json!(number);
        let state = schema.validate(&data);

        if state.is_valid() {
            println!(" - number '{}' is valid", number);
        } else {
            println!(" - number '{}' is invalid", number);
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
