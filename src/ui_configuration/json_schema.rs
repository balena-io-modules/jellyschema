use crate::dsl::schema::PropertyList;
use crate::dsl::schema::SourceSchema;
use crate::ui_configuration::properties::serialize_property_list;
use serde::ser::SerializeMap;
use serde::Serialize;
use serde::Serializer;

const SCHEMA_URL: &str = "http://json-schema.org/draft-04/schema#";

pub struct JsonSchema<'a> {
    version: u64,
    schema_url: &'a str,
    // FIXME: make recursive
    type_spec: String,
    title: &'a str,
    properties: Option<&'a PropertyList>,
}

impl<'a> Serialize for JsonSchema<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("$schema", &self.schema_url)?;
        map.serialize_entry("$$version", &self.version)?;
        map.serialize_entry("type", &self.type_spec)?;
        map.serialize_entry("title", &self.title)?;

        if self.properties.is_some() {
            serialize_property_list(&self.properties.unwrap(), &mut map)?;
        }

        map.end()
    }
}

impl<'a> From<&'a SourceSchema> for JsonSchema<'a> {
    fn from(schema: &'a SourceSchema) -> Self {
        let property_list = schema.property_list.as_ref();
        JsonSchema {
            properties: property_list,
            title: &schema.title,
            type_spec: "object".to_string(),
            version: schema.version,
            schema_url: SCHEMA_URL,
        }
    }
}
