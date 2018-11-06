use crate::dsl::compiler::compile;
use crate::dsl::compiler::CompilationError;
use crate::dsl::compiler::CompiledSchema;
use crate::ui_configuration::JsonSchema;
use crate::ui_configuration::UiObject;

use serde_json::Map;

pub struct Generator {
    compiled_schema: CompiledSchema,
}

impl Generator {
    pub fn with(yaml: serde_yaml::Value) -> Result<Self, CompilationError> {
        Ok(Generator::new(compile(yaml)?))
    }

    fn new(compiled_schema: CompiledSchema) -> Self {
        Generator { compiled_schema }
    }

    pub fn generate(self) -> (serde_json::Value, serde_json::Value) {
        let source_schema = self.compiled_schema.compiled();
        let json_schema = JsonSchema::from(&source_schema);
        let ui_object = UiObject::from(&source_schema);
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
