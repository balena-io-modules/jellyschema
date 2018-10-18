use crate::dsl::compiler::compile;
use crate::dsl::compiler::CompiledSchema;
use crate::dsl::validation;
use serde_derive::Serialize;
use serde_json::Map;

pub struct Generator {
    compiled_schema: CompiledSchema,
}

impl Generator {
    pub fn with(yaml: serde_yaml::Value) -> Result<Self, validation::Error> {
        Ok(Generator::new(compile(yaml)?))
    }

    fn new(compiled_schema: CompiledSchema) -> Self {
        Generator { compiled_schema }
    }

    pub fn generate(self) -> (serde_json::Value, serde_json::Value) {
        let schema = JsonSchema {
            version: self.compiled_schema.version,
            title: self.compiled_schema.title.to_string(),
            type_spec: ObjectType::Object,
            schema_url: "http://json-schema.org/draft-04/schema#".to_string(),
        };

        (
            serde_json::to_value(schema).expect("Internal error: inconsistent schema"),
            serde_json::Value::Object(Map::new()),
        )
    }
}

#[derive(Serialize)]
enum ObjectType {
    #[serde(rename = "object")]
    Object,
}

#[derive(Serialize)]
struct JsonSchema {
    #[serde(rename = "$$version")]
    version: u64,
    #[serde(rename = "type")]
    type_spec: ObjectType,
    #[serde(rename = "$schema")]
    schema_url: String,
    #[serde(rename = "title")]
    title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hardcode_a_type() {
        let generator = Generator::new(CompiledSchema::empty());

        let (json_schema, _) = generator.generate();

        assert_eq!(json_schema["type"], "object");
    }

    #[test]
    fn hardcode_a_schema_url() {
        let generator = Generator::new(CompiledSchema::empty());

        let (json_schema, _) = generator.generate();

        assert_eq!(json_schema["$schema"], "http://json-schema.org/draft-04/schema#");
    }

    #[test]
    fn pass_version_through() {
        let schema = CompiledSchema::with("", 21);
        let generator = Generator::new(schema);

        let (json_schema, _) = generator.generate();

        assert_eq!(json_schema["$$version"], 21);
    }

    #[test]
    fn pass_title_through() {
        let schema = CompiledSchema::with("some title", 1);
        let generator = Generator::new(schema);

        let (json_schema, _) = generator.generate();

        assert_eq!(json_schema["title"], "some title");
    }

    #[test]
    fn generate_ui_object() {
        let generator = Generator::new(CompiledSchema::empty());

        let (_, ui_object) = generator.generate();

        assert!(ui_object.is_object());
    }

    #[test]
    fn generate_json_schema() {
        let generator = Generator::new(CompiledSchema::empty());

        let (json_schema, _) = generator.generate();

        assert!(json_schema.is_object());
    }

    impl CompiledSchema {
        fn empty() -> Self {
            CompiledSchema {
                title: String::new(),
                version: 0,
            }
        }

        fn with(title: &str, version: u64) -> Self {
            CompiledSchema {
                title: title.to_string(),
                version,
            }
        }
    }
}
