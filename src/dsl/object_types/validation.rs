use crate::dsl::compiler::validator::Validate;
use crate::dsl::compiler::validator::ValidationError;
use crate::dsl::object_types::TypeDefinition;

impl Validate<TypeDefinition> for TypeDefinition {
    fn validate(&self) -> Result<(), ValidationError> {
        for enumeration_values in &self.enumeration_values {
            for enumeration_value in &enumeration_values.possible_values {
                enumeration_value.validate()?
            }
        }
        Ok(())
    }
}
