use crate::dsl::schema::deserialization::deserialize_root;
use crate::dsl::schema::SourceSchema;
use yaml_merge_keys::merge_keys_serde;

pub struct CompiledSchema {
    schema: SourceSchema,
}

#[derive(Debug)]
pub struct CompilationError {
    message: String,
}

impl CompilationError {
    pub fn with_message(message: &str) -> Self {
        CompilationError {
            message: message.to_string(),
        }
    }
}

impl From<serde_yaml::Error> for CompilationError {
    fn from(source: serde_yaml::Error) -> Self {
        CompilationError {
            message: source.to_string(),
        }
    }
}

impl From<yaml_merge_keys::Error> for CompilationError {
    fn from(source: yaml_merge_keys::Error) -> Self {
        CompilationError {
            message: source.to_string(),
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

pub fn compile(schema: serde_yaml::Value) -> Result<CompiledSchema, CompilationError> {
    let schema = merge_keys_serde(schema)?;
    let schema = deserialize_root::<serde_yaml::Error>(&schema)?;
    Ok(CompiledSchema::with(schema))
}
