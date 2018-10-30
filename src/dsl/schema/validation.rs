use crate::dsl::compiler::validator::Validate;
use crate::dsl::compiler::validator::Validated;
use crate::dsl::compiler::validator::ValidationError;
use crate::dsl::schema::SourceSchema;

impl Validate<SourceSchema> for SourceSchema {
    fn validate(self) -> Result<Validated<SourceSchema>, ValidationError> {
        if self.version == 1 {
            Ok(Validated::with(self))
        } else {
            Err(ValidationError::invalid_version(self.version))
        }
    }
}

impl ValidationError {
    pub fn invalid_version(version: u64) -> Self {
        ValidationError::with_message(&format!("Invalid version specified: {}", version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // TODO: morph into property, so that the actual unsupported version is rand
    fn fail_on_unsupported_version() {
        let schema = SourceSchema {
            title: "some title".to_string(),
            version: 13,
            property_list: None,
        };

        assert!(schema.validate().is_err());
    }
}
