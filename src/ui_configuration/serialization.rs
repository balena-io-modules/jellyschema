use crate::dsl::compiler::ObjectType;
use crate::dsl::compiler::Property;
use crate::dsl::compiler::PropertyEntry;
use crate::dsl::compiler::PropertyList;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use std::collections::HashMap;

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

pub fn serialize_property<S>(property: &Property, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_none()
}
