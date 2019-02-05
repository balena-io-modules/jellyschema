//! JSON Schema & UI Object generator
use serde_json::Map;

use crate::dsl::schema::compiler::CompilationError;
use crate::dsl::schema::compiler::compile;
use crate::dsl::schema::compiler::CompiledSchema;
use crate::output::JsonSchema;
use crate::output::UiObjectRoot;

/// JSON Schema & UI Object generator
pub struct Generator {
    compiled_schema: CompiledSchema,
}

impl Generator {
    /// Creates new generator with configuration DSL
    ///
    /// # Arguments
    ///
    /// * `yaml` - A configuration DSL
    pub fn with(yaml: serde_yaml::Value) -> Result<Generator, CompilationError> {
        Ok(Generator::new(compile(yaml)?))
    }

    fn new(compiled_schema: CompiledSchema) -> Generator {
        Generator { compiled_schema }
    }

    /// Generates JSON Schema & UI Object
    pub fn generate(self) -> (serde_json::Value, serde_json::Value) {
        let source_schema = self.compiled_schema.compiled();
        let json_schema = JsonSchema::from(source_schema.clone());
        let ui_object = UiObjectRoot::from(source_schema.clone());
        let serialized_json_schema =
            serde_json::to_value(json_schema).expect("Internal error: inconsistent schema: json schema");

        let serialized_ui_object = if !ui_object.is_empty() {
            serde_json::to_value(ui_object).expect("Internal error: inconsistent schema: ui object")
        } else {
            serde_json::Value::Object(Map::new())
        };
        (serialized_json_schema, serialized_ui_object)
    }
}
