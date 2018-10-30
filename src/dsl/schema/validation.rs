use crate::dsl::schema::SourceSchema;
use crate::dsl::validator::Validate;
use crate::dsl::validator::Validated;
use crate::dsl::validator::ValidationError;

impl Validate<SourceSchema> for SourceSchema {
    fn validate(self) -> Result<Validated<SourceSchema>, ValidationError> {
        if self.version == 1 {
            Ok(Validated::with(self))
        } else {
            Err(ValidationError::invalid_version(self.version))
        }
    }
}
