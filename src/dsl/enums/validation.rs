use crate::dsl::compiler::validator::Validate;
use crate::dsl::compiler::validator::ValidationError;
use crate::dsl::enums::EnumerationValue;

impl Validate<EnumerationValue> for EnumerationValue {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.value.is_none() {
            return Err(ValidationError::with_message("no value specified for enumeration"));
        }
        Ok(())
    }
}
