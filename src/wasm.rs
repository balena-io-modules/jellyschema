/// Web Assembly interface for the Jelly Schema.
use console_error_panic_hook::set_once as set_panic_hook_once;
use serde_json::json;
use wasm_bindgen::prelude::*;

use crate::{
    validators::{ValidateDataError, ValidationState},
    Schema, Scope,
};

#[wasm_bindgen]
pub struct JellySchema {
    schema: Schema,
    last_validation_state: ValidationState,
}

#[wasm_bindgen]
#[allow(non_snake_case)]
impl JellySchema {
    /// Instantiate new JellySchema object.
    ///
    /// # Arguments
    ///
    /// * `schema` - JellySchema as a string or an object.
    ///
    /// # Throws
    ///
    /// Constructor throws in case of invalid `schema` argument value.
    #[wasm_bindgen(constructor)]
    pub fn constructor(schema: &JsValue) -> Result<JellySchema, JsValue> {
        set_panic_hook_once();

        let scope = Scope::default();

        let schema: Schema = if schema.is_string() {
            scope
                .compile(&schema.as_string().unwrap())
                .map_err(|e| JsValue::from(format!("{}", e)))?
        } else {
            let value = schema.into_serde().map_err(|e| JsValue::from(format!("{}", e)))?;
            scope
                .compile_from_value(value)
                .map_err(|e| JsValue::from(format!("{}", e)))?
        };

        Ok(JellySchema {
            schema,
            last_validation_state: ValidationState::new(),
        })
    }

    /// Validate data against JellySchema.
    ///
    /// # Arguments
    ///
    /// * `data` - A JSON object.
    pub fn validate(&mut self, data: &JsValue) -> Result<bool, JsValue> {
        if data.is_undefined() {
            self.last_validation_state = ValidateDataError {
                data_path: "".to_string(),
                keyword: "",
                message: "unable to deserialize given data: {}".to_string(),
            }
            .into();
            return Ok(false);
        }

        match data.into_serde() {
            Ok(data) => {
                self.last_validation_state = self.schema.validate(&data);
                Ok(self.last_validation_state.is_valid())
            }
            Err(e) => {
                self.last_validation_state = ValidateDataError {
                    data_path: "".to_string(),
                    keyword: "",
                    message: format!("unable to deserialize given data: {}", e),
                }
                .into();
                Ok(false)
            }
        }
    }

    /// Return last validation errors.
    ///
    /// # Throws
    ///
    /// In case of internal error only (serialization).
    pub fn errors(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&json!(self.last_validation_state.errors()))
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }
}
