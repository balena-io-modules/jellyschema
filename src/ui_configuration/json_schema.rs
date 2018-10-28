use crate::dsl::schema::PropertyList;
use crate::dsl::types::ObjectType;
use crate::dsl::types::TypeSpec;
use crate::dsl::validation::ValidatedSchema;
use serde_derive::Serialize;

const SCHEMA_URL: &str = "http://json-schema.org/draft-04/schema#";

#[derive(Serialize)]
pub struct JsonSchema<'a> {
    #[serde(rename = "$$version")]
    version: u64,
    #[serde(rename = "$schema")]
    schema_url: &'a str,
    #[serde(rename = "type")]
    type_spec: TypeSpec,
    title: &'a str,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    properties: Option<&'a PropertyList>,
    #[serde(rename = "$$order", skip_serializing_if = "Option::is_none")]
    order: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<Vec<&'a str>>,
}

impl<'a> From<&'a ValidatedSchema> for JsonSchema<'a> {
    fn from(schema: &'a ValidatedSchema) -> Self {
        let property_names = match &schema.property_list {
            Some(list) => {
                let property_names = list.property_names.iter().map(|name| name.as_str()).collect();
                Some(property_names)
            }
            None => None,
        };

        let required_property_names = match &schema.property_list {
            Some(list) => required_property_names(list),
            None => None,
        };

        JsonSchema {
            properties: schema.property_list.as_ref(),
            title: &schema.title,
            required: required_property_names,
            order: property_names,
            type_spec: TypeSpec::Required(ObjectType::Object),
            version: schema.version,
            schema_url: SCHEMA_URL,
        }
    }
}

// TODO: associate with PropertyList type
fn required_property_names(property_list: &PropertyList) -> Option<Vec<&str>> {
    let required_property_names: Vec<&str> = property_list
        .entries
        .iter()
        .filter_map(|property_entry| match &property_entry.property.type_spec {
            Some(type_spec) => match type_spec {
                TypeSpec::Required(_) => Some(property_entry.name.as_str()),
                TypeSpec::Optional(_) => None,
            },
            None => None,
        })
        .collect();

    if !required_property_names.is_empty() {
        Some(required_property_names)
    } else {
        None
    }
}
