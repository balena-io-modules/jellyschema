use crate::dsl::compiler::validator::Validate;
use crate::dsl::enums::EnumerationValue;
use crate::dsl::compiler::validator::Validated;
use crate::dsl::compiler::validator::ValidationError;

impl Validate<EnumerationValue> for EnumerationValue {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.value.is_none() {
            return Err(ValidationError::with_message("no value specified for enumeration"));
        }
        Ok(())
    }
}