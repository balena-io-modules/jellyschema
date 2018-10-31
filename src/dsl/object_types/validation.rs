use crate::dsl::compiler::validator::Validate;
use crate::dsl::compiler::validator::ValidationError;
use crate::dsl::object_types::TypeDefinition;

impl Validate for TypeDefinition {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.constant_value.is_some() && self.enumeration_values.is_some() {
            return Err(ValidationError::with_message(
                "cannot have both enumeration values definition and constant value definition",
            ));
        }

        for enumeration_values in &self.enumeration_values {
            for enumeration_value in &enumeration_values.possible_values {
                enumeration_value.validate()?
            }
        }

        Ok(())
    }
}
