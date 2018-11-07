use crate::dsl::compiler::validator::Validate;
use crate::dsl::compiler::validator::ValidationError;
use crate::dsl::schema::PropertyEntry;
use crate::dsl::schema::SourceSchema;

impl Validate for SourceSchema {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.version != 1 {
            return Err(ValidationError::invalid_version(self.version));
        }
        for list in &self.property_list {
            for property in &list.entries {
                property.validate()?
            }
        }
        Ok(())
    }
}

impl Validate for PropertyEntry {
    fn validate(&self) -> Result<(), ValidationError> {
        match &self.property.types {
            Some(spec) => {
                for def in spec {
                    def.validate()?;
                }
                Ok(())
            }
            None => Ok(()),
        }

        // TODO: validate that if there are properties the type must be object
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
