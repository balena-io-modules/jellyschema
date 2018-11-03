use crate::dsl::compiler::validator::Validate;
use crate::dsl::compiler::validator::ValidationError;
use crate::dsl::object_types::bounds::EnumerationValue;

impl Validate for EnumerationValue {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.value.is_none() {
            return Err(ValidationError::with_message("no value specified for enumeration"));
        }
        Ok(())
    }
}
