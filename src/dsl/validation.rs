use crate::dsl::schema::SourceSchema;

pub type ValidatedSchema = SourceSchema;

pub fn validate(source_schema: SourceSchema) -> Result<ValidatedSchema, Error> {
    if source_schema.version != 1 {
        return Err(Error::invalid_version(source_schema.version));
    }
    Ok(source_schema)
}

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn invalid_version(version: u64) -> Self {
        Error {
            message: format!("Invalid version specified: {}", version),
        }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(source: serde_yaml::Error) -> Self {
        Error {
            message: source.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error {
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
            properties: None,
        };

        assert!(validate(schema).is_err());
    }
}
