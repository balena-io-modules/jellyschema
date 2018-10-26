use crate::dsl::compiler::compile;
use crate::dsl::validation;
use crate::dsl::validation::ValidatedSchema;
use crate::ui_configuration::json_schema::JsonSchema;
use crate::ui_configuration::ui_object::UiObject;

pub struct Generator {
    compiled_schema: ValidatedSchema,
}

impl Generator {
    pub fn with(yaml: serde_yaml::Value) -> Result<Self, validation::Error> {
        Ok(Generator::new(compile(yaml)?))
    }

    fn new(compiled_schema: ValidatedSchema) -> Self {
        Generator { compiled_schema }
    }

    pub fn generate(self) -> (serde_json::Value, serde_json::Value) {
        let schema = JsonSchema::from(&self.compiled_schema);
        let ui_object = UiObject::from(&self.compiled_schema);

        (
            serde_json::to_value(schema).expect("Internal error: inconsistent schema: json schema"),
            serde_json::to_value(ui_object).expect("Internal error: inconsistent schema: ui object"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::schema::SourceSchema;
    use crate::dsl::validation::validate;

    #[test]
    fn hardcode_a_type() -> Result<(), validation::Error> {
        let generator = Generator::new(validate(SourceSchema::empty())?);

        let (json_schema, _) = generator.generate();

        assert_eq!(json_schema["type"], "object");
        Ok(())
    }

    #[test]
    fn pass_title_through() -> Result<(), validation::Error> {
        let schema = validate(SourceSchema::with("some title", 1))?;
        let generator = Generator::new(schema);

        let (json_schema, _) = generator.generate();

        assert_eq!(json_schema["title"], "some title");
        Ok(())
    }

    #[test]
    fn generate_ui_object() -> Result<(), validation::Error> {
        let generator = Generator::new(validate(SourceSchema::empty())?);

        let (_, ui_object) = generator.generate();

        assert!(ui_object.is_object());
        Ok(())
    }

    #[test]
    fn generate_json_schema() -> Result<(), validation::Error> {
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
