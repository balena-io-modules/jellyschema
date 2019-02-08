use console_error_panic_hook::set_once as set_panic_hook_once;
use serde_json::{json, Value};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use crate::{
    generator::generate_json_ui_schema,
    schema::Schema,
    validator::{ValidationError, ValidationState, Validator},
};

#[wasm_bindgen]
pub struct JellySchema {
    schema: Schema,
    json_ui_schema: Option<(Value, Value)>,
    last_validation_state: ValidationState,
}

#[wasm_bindgen]
#[allow(non_snake_case)]
impl JellySchema {
    /// Instantiates new JellySchema object
    ///
    /// # Arguments
    ///
    /// * `schema` - JellySchema as a string or an object
    ///
    /// # Throws
    ///
    /// Constructor throws in case of invalid `schema` argument value.
    #[wasm_bindgen(constructor)]
    pub fn constructor(schema: &JsValue) -> Result<JellySchema, JsValue> {
        set_panic_hook_once();

        let schema: Schema = if schema.is_string() {
            Schema::from_str(&schema.as_string().unwrap()).map_err(|e| JsValue::from(format!("{}", e)))?
        } else {
            schema.into_serde().map_err(|e| JsValue::from(format!("{}", e)))?
        };

        Ok(JellySchema {
            schema,
            json_ui_schema: None,
            last_validation_state: ValidationState::new(),
        })
    }

    /// Validates data against JellySchema
    ///
    /// # Arguments
    ///
    /// * `data` - A JSON object
    pub fn validate(&mut self, data: &JsValue) -> Result<bool, JsValue> {
        match data.into_serde() {
            Ok(data) => {
                self.last_validation_state = self.schema.validate(Some(&data));
                Ok(self.last_validation_state.is_valid())
            }
            Err(e) => {
                self.last_validation_state =
                    ValidationError::new("", "", "", format!("unable to deserialize given data: {}", e)).into();
                Ok(false)
            }
        }
    }

    /// Generates JSON Schema & UI Schema object
    ///
    /// ```js
    /// {
    ///     "jsonSchema": {...},
    ///     "uiSchema": {...}
    /// }
    /// ```
    ///
    /// # Throws
    ///
    /// In case of internal error only (serialization).
    pub fn jsonAndUiSchema(&mut self) -> Result<JsValue, JsValue> {
        if self.json_ui_schema.is_none() {
            self.json_ui_schema = Some(generate_json_ui_schema(&self.schema));
        }

        let schemas = self.json_ui_schema.as_ref().unwrap();

        JsValue::from_serde(&json!({
            "jsonSchema": schemas.0,
            "uiSchema": schemas.1,
        }))
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// Returns last validation errors
    ///
    /// # Throws
    ///
    /// In case of internal error only (serialization).
    pub fn errors(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&json!(self.last_validation_state.errors()))
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }
}

/// Generates JSON and UI schema object
///
/// ```js
/// {
///     "jsonSchema": {...},
///     "uiSchema": {...}
/// }
/// ```
///
/// # Arguments
///
/// * `schema` - Jelly Schema as an object or a string
///
/// # Throws
///
/// If the input schema is invalid or in case of internal error (serialization).
#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn generateJsonAndUiSchema(schema: &JsValue) -> Result<JsValue, JsValue> {
    // This is okay for now, but in the future, when the `constructor`
    // will be much more expensive (doing lot of other things), we will have to
    // replace it with direct calls to `generate_json_ui_schema`, etc.
    JellySchema::constructor(schema)?.jsonAndUiSchema()
}
