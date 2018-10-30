use crate::dsl::compiler::compile;
use crate::dsl::schema::SourceSchema;
use crate::dsl::validator;
use crate::dsl::validator::Validated;
use crate::ui_configuration::json_schema::JsonSchema;
use crate::ui_configuration::ui_object::UiObject;
use serde_json::Map;

pub struct Generator {
    compiled_schema: Validated<SourceSchema>,
}

impl Generator {
    pub fn with(yaml: serde_yaml::Value) -> Result<Self, validator::ValidationError> {
        Ok(Generator::new(compile(yaml)?))
    }

    fn new(compiled_schema: Validated<SourceSchema>) -> Self {
        Generator { compiled_schema }
    }

    pub fn generate(self) -> (serde_json::Value, serde_json::Value) {
        let source_schema = self.compiled_schema.validated();
        let json_schema = JsonSchema::from(source_schema);
        let ui_object = UiObject::from(source_schema);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::schema::SourceSchema;
    use crate::dsl::validator::validate;

    #[test]
    fn hardcode_root_level_type() -> Result<(), validator::ValidationError> {
        let generator = Generator::new(validate(SourceSchema::empty())?);

        let (json_schema, _) = generator.generate();
        assert_eq!(json_schema["type"], "object");
        Ok(())
    }

    #[test]
    fn pass_title_through() -> Result<(), validator::ValidationError> {
        let schema = validate(SourceSchema::with("some title", 1))?;
        let generator = Generator::new(schema);

        let (json_schema, _) = generator.generate();

        assert_eq!(json_schema["title"], "some title");
        Ok(())
    }

    #[test]
    fn generate_ui_object() -> Result<(), validator::ValidationError> {
        let generator = Generator::new(validate(SourceSchema::empty())?);

        let (_, ui_object) = generator.generate();

        assert!(ui_object.is_object());
        Ok(())
    }

    #[test]
    fn generate_json_schema() -> Result<(), validator::ValidationError> {
        let generator = Generator::new(validate(SourceSchema::empty())?);

        let (json_schema, _) = generator.generate();

        assert!(json_schema.is_object());
        Ok(())
    }

    impl SourceSchema {
        fn empty() -> Self {
            SourceSchema {
                title: String::new(),
                version: 1,
                property_list: None,
            }
        }

        fn with(title: &str, version: u64) -> Self {
            SourceSchema {
                title: title.to_string(),
                version,
                property_list: None,
            }
        }
    }
}
