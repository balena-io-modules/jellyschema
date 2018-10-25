use crate::dsl::compiler::ObjectType;
use crate::dsl::compiler::Property;
use crate::dsl::compiler::PropertyEntry;
use crate::dsl::compiler::PropertyList;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::collections::HashMap;

impl Serialize for Property {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        if self.title.is_some() {
            map.serialize_entry("title", &self.title.clone().unwrap())?;
        }

        if self.type_spec.is_some() {
            let type_spec = self.type_spec.clone().unwrap();
            match type_spec {
                ObjectType::Object => map.serialize_entry("type", "object")?,
                ObjectType::Hostname => {
                    map.serialize_entry("type", "string")?;
                    map.serialize_entry("format", "hostname")?;
                }
            };
        };

        map.end()
    }
}

pub fn serialize_type<S>(object_type: &Option<ObjectType>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_type {
        Some(object_type) => match object_type {
            ObjectType::Object => serializer.serialize_str("object"),
            ObjectType::Hostname => serializer.serialize_str("hostname"),
        },
        None => serializer.serialize_none(),
    }
}

pub fn serialize_property_list<S>(property_list: &Option<PropertyList>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match property_list {
        Some(list) => {
            let mut map = serializer.serialize_map(Some(list.entries.iter().count()))?;
            for entry in list.clone().entries {
                map.serialize_entry(&entry.name, &entry.property);
            }
            map.end()
        }
        None => serializer.serialize_none(),
    }
}

