use crate::dsl::compiler::validator::Validate;
use crate::dsl::compiler::validator::ValidationError;
use crate::dsl::object_types::TypeDefinition;

impl Validate for TypeDefinition {
    fn validate(&self) -> Result<(), ValidationError> {

        Ok(())
    }
}
