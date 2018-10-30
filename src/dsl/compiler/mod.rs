pub mod normalizer;
pub mod validator;

use crate::dsl::compiler::normalizer::normalize;
use crate::dsl::compiler::validator::validate;
use crate::dsl::compiler::validator::ValidationError;
use crate::dsl::schema::SourceSchema;

pub struct CompiledSchema {
    schema: SourceSchema,
}

#[derive(Debug)]
pub struct CompilationError {
    message: String,
}

impl From<ValidationError> for CompilationError {
    fn from(error: ValidationError) -> Self {
        CompilationError {
            message: error.into_message(),
        }
    }
}

impl CompiledSchema {
    pub fn with(schema: SourceSchema) -> Self {
        CompiledSchema { schema }
    }

    pub fn compiled(self) -> SourceSchema {
        self.schema
    }
}

pub fn compile(schema: serde_yaml::Value) -> Result<CompiledSchema, validator::ValidationError> {
    let schema: SourceSchema = serde_yaml::from_value(schema)?;
    let normalized_schema = normalize(schema).normalized();
    let validated_schema = validate(normalized_schema)?;
    Ok(CompiledSchema::with(validated_schema.validated()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml::Mapping;

    const SOME_TITLE: &str = "some title";

    #[test]
    fn pass_title_through() -> Result<(), validator::ValidationError> {
        let schema = yaml_schema_with(SOME_TITLE, 1);

        assert_eq!(compile(schema)?.compiled().title, SOME_TITLE);
        Ok(())
    }

    #[test]
    fn fail_on_missing_title() {
        let mut schema = Mapping::new();
        schema.insert("version".into(), 1.into());
        let schema = serde_yaml::Value::Mapping(schema);

        assert!(compile(schema).is_err());
    }

    fn yaml_schema_with(title: &str, version: u64) -> serde_yaml::Value {
        let mut schema = Mapping::new();
        schema.insert("title".into(), title.into());
        schema.insert("version".into(), version.into());
        serde_yaml::Value::Mapping(schema)
    }
}
