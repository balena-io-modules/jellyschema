use std::collections::HashMap;

// https://github.com/rustwasm/console_error_panic_hook#readme
pub use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;

use crate::output::generator::Generator;

/// Evaluates the whole JSON
#[wasm_bindgen]
pub fn generate_ui(yaml_text: JsValue) -> Result<JsValue, JsValue> {
    // use console.log for nice errors from Rust-land
    console_error_panic_hook::set_once();

    let yaml_text = yaml_text
        .as_string()
        .ok_or_else(|| JsValue::from("yaml given is not a string"))?;
    let schema = serde_yaml::from_str(&yaml_text).map_err(|e| JsValue::from(format!("{:#?}", e)))?;

    let (json_schema, ui_object) = Generator::with(schema)
        .map_err(|e| JsValue::from(format!("{:#?}", e)))?
        .generate();

    let mut result = HashMap::new();
    result.insert("json_schema", json_schema);
    result.insert("ui_object", ui_object);

    let result = JsValue::from_serde(&result).map_err(|e| JsValue::from(format!("{:#?}", e)))?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    use crate::wasm::generate_ui;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn run_in_browser() {
        let yaml = JsValue::from(
            r#"
            version: 1
            title: Empty schema
        "#,
        );

        let expected_result = json!(
        {
            "json_schema": {
                "$$version": 1,
                "$schema": "http://json-schema.org/draft-04/schema#",
                "title": "Empty schema",
                "type": "object",
                "additionalProperties": false
            },
            "ui_object": {}
        }
        );

        let generated_ui: serde_json::Value = generate_ui(yaml).unwrap().into_serde().unwrap();

        assert_eq!(generated_ui, expected_result);
    }
}
