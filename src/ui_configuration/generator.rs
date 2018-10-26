use crate::dsl::compiler::compile;
use crate::dsl::schema::PropertyList;
use crate::dsl::types::ObjectType;
use crate::dsl::types::TypeSpec;
use crate::dsl::validation;
use crate::dsl::validation::ValidatedSchema;
use serde_derive::Serialize;
use std::collections::HashMap;

const SCHEMA_URL: &str = "http://json-schema.org/draft-04/schema#";

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

#[derive(Serialize)]
struct JsonSchema {
    #[serde(rename = "$$version")]
    version: u64,
    #[serde(rename = "$schema")]
    schema_url: String,
    #[serde(rename = "type")]
    type_spec: TypeSpec,
    title: String,
    #[serde(
        rename = "properties",
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::ui_configuration::to_json_schema::serialize_property_list"
    )]
    properties: Option<PropertyList>,
    #[serde(rename = "$$order", skip_serializing_if = "Option::is_none")]
    order: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<Vec<String>>,
}

impl From<&ValidatedSchema> for JsonSchema {
    fn from(schema: &ValidatedSchema) -> Self {
        let property_names = match schema.properties {
            Some(ref list) => Some(list.clone().property_names),
            None => None,
        };

        JsonSchema {
            version: schema.version,
            title: schema.title.to_string(),
            properties: schema.properties.clone(),
            required: property_names.clone(),
            order: property_names.clone(),
            type_spec: TypeSpec::Required(ObjectType::Object),
            schema_url: SCHEMA_URL.to_string(),
        }
    }
}

#[derive(Serialize)]
struct UiObject(HashMap<String, UiObjectProperty>);

impl From<&ValidatedSchema> for UiObject {
    fn from(schema: &ValidatedSchema) -> Self {
        let mut ui_object_entries = HashMap::new();

        if schema.properties.clone().is_some() {
            for property_entry in schema.properties.clone().unwrap().entries {
                ui_object_entries.insert(
                    property_entry.name.to_string(),
                    UiObjectProperty {
                        help: property_entry.property.help,
                        warning: property_entry.property.warning,
                        description: property_entry.property.description,
                    },
                );
            }
        }
        UiObject(ui_object_entries)
    }
}

#[derive(Serialize)]
struct UiObjectProperty {
    #[serde(rename = "ui:help")]
    help: Option<String>,
    #[serde(rename = "ui:warning")]
    warning: Option<String>,
    #[serde(rename = "ui:description")]
    description: Option<String>,
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
    fn hardcode_a_schema_url() -> Result<(), validation::Error> {
        let generator = Generator::new(validate(SourceSchema::empty())?);

        let (json_schema, _) = generator.generate();

        assert_eq!(json_schema["$schema"], SCHEMA_URL);
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
                properties: None,
            }
        }

        fn with(title: &str, version: u64) -> Self {
            SourceSchema {
                title: title.to_string(),
                version,
                properties: None,
            }
        }
    }
}
