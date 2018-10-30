use crate::dsl::schema::{Property, PropertyList};
use crate::dsl::types::{ObjectType, TypeSpec};
use serde::ser::{Error, Serialize, SerializeMap, Serializer};
use crate::dsl::types::EnumerationValue;

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
            serialize_object_type(&type_spec.inner(), &mut map)?;
        }

        let mut enumeration_possible_values = vec![];
        for some_enumeration_value in &self.type_information.enumeration_values {
            for enumeration_value in &some_enumeration_value.possible_values {
                enumeration_possible_values.push(enumeration_value);
            }
        }

        if !enumeration_possible_values.is_empty() {
            map.serialize_entry("oneOf", &enumeration_possible_values)?;
        }

        map.end()
    }
}

// TODO: use json display struct instead
impl Serialize for EnumerationValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut map = serializer.serialize_map(None)?;
        if self.display_information.title.is_some() {
            map.serialize_entry("title", &self.display_information.title)?;
            map.serialize_entry("enum", &vec![&self.value])?;
        }
        serialize_object_type(&self.type_spec.inner(), &mut map);

        map.end()
    }
}

impl Serialize for TypeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        serialize_object_type(self.inner(), &mut map)?;
        map.end()
    }
}

fn serialize_object_type<O, E, S>(object_type: &ObjectType, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,

{
    let result = match object_type {
        ObjectType::Object => map.serialize_entry("type", "object")?,
        ObjectType::String => map.serialize_entry("type", "string")?,
        ObjectType::Hostname => {
            map.serialize_entry("type", "string")?;
            map.serialize_entry("format", "hostname")?
        }
    };
    Ok(result)
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
