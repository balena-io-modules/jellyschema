use crate::dsl_schema::CompiledSchema;
use serde_json::map::Map;

pub struct Generator;

impl Generator {
    pub fn new(compiled_schema: CompiledSchema) -> Self {
        Generator {}
    }

    pub fn generate(self) -> (serde_json::Value, serde_json::Value) {
        (
            serde_json::Value::Object(Map::new()),
            serde_json::Value::Object(Map::new()),
        )
    }
}

#[cfg(test)]
mod generated_json_schema {
    mod should {
        use crate::dsl_schema::*;
        use crate::ui_config::*;

        #[test]
        fn have_a_version() {
            let schema = CompiledSchema::new();
            let generator = Generator::new(schema);

            let (json_schema, _) = generator.generate();

            assert_eq!(json_schema["version"], 1);
        }
    }
}

#[cfg(test)]
mod generator {
    mod should {
        use crate::dsl_schema::*;
        use crate::ui_config::*;

        #[test]
        fn generate_ui_object() {
            let schema = CompiledSchema::new();
            let generator = Generator::new(schema);

            let (_, ui_object) = generator.generate();

            assert!(ui_object.is_object());
        }

        #[test]
        fn generate_json_schema() {
            let schema = CompiledSchema::new();
            let generator = Generator::new(schema);

            let (json_schema, _) = generator.generate();

            assert!(json_schema.is_object());
        }
    }
}
