//! Data type value generator.
use serde_json::Value;

use crate::Schema;

pub type Result<T> = std::result::Result<T, GenerateValueError>;

/// Value generator error.
pub struct GenerateValueError {
    /// Data type name.
    pub data_type: &'static str,
    /// Error message.
    pub message: String,
}

/// Data type value generator trait object.
pub type BoxedGenerator = Box<dyn Generator>;

/// Data type value generator interface.
pub trait Generator {
    /// Generate random data type value.
    fn generate(&self, schema: &Schema) -> Result<Value>;
}
