use crate::dsl::schema::SourceSchema;

pub struct Validated<T> {
    validated: T,
}

impl<T> Validated<T> {
    pub fn with(value: T) -> Self {
        Validated { validated: value }
    }
}

impl<T> Validated<T> {
    pub fn validated(&self) -> &T {
        &self.validated
    }
}

pub trait Validate<T> {
    fn validate(self) -> Result<Validated<T>, ValidationError>;
}

pub fn validate(source_schema: SourceSchema) -> Result<Validated<SourceSchema>, ValidationError> {
    Ok(source_schema.validate()?)
}

#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn invalid_version(version: u64) -> Self {
        ValidationError {
            message: format!("Invalid version specified: {}", version),
        }
    }
}

impl From<serde_yaml::Error> for ValidationError {
    fn from(source: serde_yaml::Error) -> Self {
        ValidationError {
            message: source.to_string(),
        }
    }
}

impl From<serde_json::Error> for ValidationError {
    fn from(source: serde_json::Error) -> Self {
        ValidationError {
            message: source.to_string(),
        }
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

        assert!(validate(schema).is_err());
    }
}
