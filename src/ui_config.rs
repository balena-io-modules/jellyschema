use crate::dsl_schema::CompiledSchema;
use serde_json::map::Map;
use serde_json::Number;
use serde_json::Value;

pub struct Generator;

impl Generator {
    pub fn new(compiled_schema: CompiledSchema) -> Self {
        Generator {}
    }

    pub fn generate(self) -> (serde_json::Value, serde_json::Value) {
        let mut schema = Map::new();
        schema.insert("$$version".to_string(), 1.into());
        schema.insert("type".to_string(), "object".into());

        (serde_json::Value::Object(schema), serde_json::Value::Object(Map::new()))
    }
}

#[cfg(test)]
mod generated_json_schema {
    mod must {
        use crate::dsl_schema::*;
        use crate::ui_config::*;

        #[test]
        fn have_a_version() {
            let schema = CompiledSchema::empty();
            let generator = Generator::new(schema);

            let (json_schema, _) = generator.generate();

            assert_eq!(json_schema["$$version"], 1);
        }

        #[test]
        fn have_a_type() {
            let schema = CompiledSchema::empty();
            let generator = Generator::new(schema);

            let (json_schema, _) = generator.generate();

            assert_eq!(json_schema["type"], "object");
        }
    }
}

#[cfg(test)]
mod generator {
    mod must {
        use crate::dsl_schema::*;
        use crate::ui_config::*;

        #[test]
        fn generate_ui_object() {
            let schema = CompiledSchema::empty();
            let generator = Generator::new(schema);

            let (_, ui_object) = generator.generate();

            assert!(ui_object.is_object());
        }

        #[test]
        fn generate_json_schema() {
            let schema = CompiledSchema::empty();
            let generator = Generator::new(schema);

            let (json_schema, _) = generator.generate();

            assert!(json_schema.is_object());
        }
    }
}
