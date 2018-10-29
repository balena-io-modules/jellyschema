use crate::dsl::schema::{Property, PropertyList};
use crate::dsl::types::{ObjectType, TypeSpec};
use serde::ser::{Error, Serialize, SerializeMap, Serializer};

impl Serialize for Property {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        for title in &self.display_information.title {
            map.serialize_entry("title", &title)?;
        }

        for type_spec in &self.type_information.spec {
            match &type_spec.inner() {
                ObjectType::Object => map.serialize_entry("type", "object")?,
                ObjectType::String => map.serialize_entry("type", "string")?,
                ObjectType::Hostname => {
                    map.serialize_entry("type", "string")?;
                    map.serialize_entry("format", "hostname")?;
                }
            };
        }

        map.end()
    }
}

// TODO: merge into the code above - right now there are 2 paths through the serialization - one for root one for others
// make it one
impl Serialize for TypeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.inner() {
            ObjectType::Object => serializer.serialize_str("object"),
            _ => Err(Error::custom("unknown object type")),
        }
    }
}

impl Serialize for PropertyList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries_count = self.entries.iter().count();
        let mut map = serializer.serialize_map(Some(entries_count))?;
        for entry in &self.entries {
            map.serialize_entry(&entry.name, &entry.property)?;
        }
        map.end()
    }
}
