use crate::dsl::schema::SourceSchema;

pub struct Validated<T> {
    validated: T,
}

impl<T> Validated<T> {
    pub fn from(value: T) -> Self {
        Validated { validated: value }
    }
    pub fn validated(self) -> T {
        self.validated
    }
}

pub trait Validate<T>
{
    fn validate(&self) -> Result<(), ValidationError>;
}

pub fn validate(source_schema: SourceSchema) -> Result<Validated<SourceSchema>, ValidationError> {
    source_schema.validate()?;
    Ok(Validated::from(source_schema))
}

#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn with_message(message: &str) -> Self {
        ValidationError {
            message: message.to_string(),
        }
    }
    pub fn into_message(self) -> String {
        self.message
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
