pub type UIObject = serde_json::Value;
pub type JsonSchema = serde_json::Value;

pub mod generator {
    use super::JsonSchema;
    use super::UIObject;
    use crate::dsl_schema::compiler::CompiledSchema;
    use serde_derive::Serialize;
    use serde_json::map::Map;

    pub struct Generator<'a> {
        compiled_schema: &'a CompiledSchema,
    }

    impl<'a> Generator<'a> {
        pub fn new(compiled_schema: &'a CompiledSchema) -> Self {
            Generator { compiled_schema }
        }

        pub fn generate(self) -> (JsonSchema, UIObject) {
            let schema = OutputJsonSchema {
                version: self.compiled_schema.version(),
                type_spec: OutputObjectType::Object,
                schema_url: "http://json-schema.org/draft-04/schema#".to_string(),
                title: self.compiled_schema.title().to_string(),
            };

            (
                serde_json::to_value(schema).expect("Internal error: inconsistent schema"),
                serde_json::Value::Object(Map::new()),
            )
        }
    }

    #[derive(Serialize)]
    enum OutputObjectType {
        #[serde(rename = "object")]
        Object,
    }

    #[derive(Serialize)]
    struct OutputJsonSchema {
        #[serde(rename = "$$version")]
        version: u64,
        #[serde(rename = "type")]
        type_spec: OutputObjectType,
        #[serde(rename = "$schema")]
        schema_url: String,
        #[serde(rename = "title")]
        title: String,
    }

    #[cfg(test)]
    mod must {
        use super::Generator;
        use crate::dsl_schema::compiler::CompiledSchema;

        #[test]
        fn hardcode_a_type() {
            let schema = CompiledSchema::empty();
            let generator = Generator::new(&schema);

            let (json_schema, _) = generator.generate();

            assert_eq!(json_schema["type"], "object");
        }

        #[test]
        fn hardcode_a_schema_url() {
            let schema = CompiledSchema::empty();
            let generator = Generator::new(&schema);

            let (json_schema, _) = generator.generate();

            assert_eq!(json_schema["$schema"], "http://json-schema.org/draft-04/schema#");
        }

        #[test]
        fn pass_version_through() {
            let schema = CompiledSchema::with("", 21);
            let generator = Generator::new(&schema);

            let (json_schema, _) = generator.generate();

            assert_eq!(json_schema["$$version"], 21);
        }

        #[test]
        fn pass_title_through() {
            let schema = CompiledSchema::with("some title", 1);
            let generator = Generator::new(&schema);

            let (json_schema, _) = generator.generate();

            assert_eq!(json_schema["title"], "some title");
        }

        #[test]
        fn generate_ui_object() {
            let schema = CompiledSchema::empty();
            let generator = Generator::new(&schema);

            let (_, ui_object) = generator.generate();

            assert!(ui_object.is_object());
        }

        #[test]
        fn generate_json_schema() {
            let schema = CompiledSchema::empty();
            let generator = Generator::new(&schema);

            let (json_schema, _) = generator.generate();

            assert!(json_schema.is_object());
        }
    }
}
