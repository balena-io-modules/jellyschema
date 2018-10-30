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
    // FIXME: make recursive
    type_spec: String,
    title: &'a str,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    properties: Option<&'a PropertyList>,
    #[serde(rename = "$$order", skip_serializing_if = "Vec::is_empty")]
    order: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    required: Vec<&'a str>,
}

impl<'a> From<&'a ValidatedSchema> for JsonSchema<'a> {
    fn from(schema: &'a ValidatedSchema) -> Self {
        let property_list = schema.property_list.as_ref();
        JsonSchema {
            properties: property_list,
            title: &schema.title,
            required: property_list.map_or(vec![], |list| list.required_property_names()),
            order: property_list.map_or(vec![], |list| list.property_names()),
            type_spec: "object".to_string(),
            version: schema.version,
            schema_url: SCHEMA_URL,
        }
    }
}
