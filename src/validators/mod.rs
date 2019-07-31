//! Data validators.
use serde::Serialize;
use serde_json::Value;

use crate::WalkContext;

pub(crate) mod const_;
pub(crate) mod enum_;
pub(crate) mod exclusive_max;
pub(crate) mod exclusive_min;
pub(crate) mod items;
pub(crate) mod max;
pub(crate) mod max_items;
pub(crate) mod max_length;
pub(crate) mod min;
pub(crate) mod min_items;
pub(crate) mod min_length;
pub(crate) mod multiple_of;
pub(crate) mod pattern;
pub(crate) mod properties;
pub(crate) mod type_;
pub(crate) mod unique_items;

/// Data validation error.
#[derive(Debug, Serialize)]
pub struct ValidateDataError {
    /// Data path in the JSON.
    #[serde(rename = "dataPath")]
    pub data_path: String,
    /// Schema keyword, which failed.
    pub keyword: &'static str,
    /// Error message.
    pub message: String,
}

/// Validation state.
#[derive(Debug, Default)]
pub struct ValidationState {
    /// List of validation errors.
    errors: Vec<ValidateDataError>,
}

impl ValidationState {
    /// Create new validation state (without errors).
    pub fn new() -> ValidationState {
        ValidationState { errors: vec![] }
    }

    /// Append another state errors.
    pub fn append(&mut self, state: ValidationState) {
        let mut state = state;
        self.errors.append(&mut state.errors);
    }

    /// Check if there're no errors.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if there're errors.
    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    /// List of validation errors (can be empty).
    pub fn errors(&self) -> &[ValidateDataError] {
        &self.errors
    }
}

impl From<ValidateDataError> for ValidationState {
    fn from(e: ValidateDataError) -> Self {
        ValidationState { errors: vec![e] }
    }
}

/// Data validator interface.
pub trait Validator {
    /// Validate data in the given context.
    fn validate(&self, _data: &Value, _ctx: &WalkContext) -> ValidationState {
        ValidationState::new()
    }
}

/// Data validator trait object.
pub type BoxedValidator = Box<dyn Validator>;

/// Validation helpers.
pub trait WalkContextExt {
    /// Helper to instantiate data validation error.
    fn validation_error<S: Into<String>>(&self, keyword: &'static str, message: S) -> ValidateDataError;
}

impl<'a> WalkContextExt for WalkContext<'a> {
    fn validation_error<S: Into<String>>(&self, keyword: &'static str, message: S) -> ValidateDataError {
        ValidateDataError {
            data_path: self.json_path(),
            keyword,
            message: message.into(),
        }
    }
}
