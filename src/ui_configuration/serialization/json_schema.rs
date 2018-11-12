use crate::dsl::schema::SourceSchema;
use crate::ui_configuration::serialization::properties::serialize_property;
use crate::ui_configuration::JsonSchema;
use serde::ser::SerializeMap;
use serde::Serialize;
use serde::Serializer;

const SCHEMA_URL: &str = "http://json-schema.org/draft-04/schema#";

impl<'a> Serialize for JsonSchema<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("$schema", &self.schema_url)?;
        map.serialize_entry("$$version", &self.version)?;

        if let Some(property) = self.root {
            serialize_property(property, &mut map)?
        }

        map.end()
    }
}

impl<'a> From<&'a SourceSchema> for JsonSchema<'a> {
    fn from(schema: &'a SourceSchema) -> Self {
        JsonSchema {
            root: schema.self_property.as_ref(),
            version: schema.version,
            schema_url: SCHEMA_URL,
        }
    }
}
