use crate::dsl::compiler::validator::Validate;
use crate::dsl::compiler::validator::ValidationError;
use crate::dsl::object_types::ObjectType;

impl Validate for ObjectType {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}
