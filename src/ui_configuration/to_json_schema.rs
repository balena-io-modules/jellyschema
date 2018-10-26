use crate::dsl::compiler::ObjectType;
use crate::dsl::compiler::Property;
use crate::dsl::compiler::PropertyList;
use crate::dsl::compiler::TypeSpec;
use serde::ser::{Serialize, SerializeMap, Serializer};

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
            let object_type = type_spec.unwrap();
            match object_type {
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

impl Serialize for TypeSpec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.clone().unwrap() {
            ObjectType::Object => serializer.serialize_str("object"),
            _ => Err(serde::ser::Error::custom("unknown object type")),
        }
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
                map.serialize_entry(&entry.name, &entry.property)?;
            }
            map.end()
        }
        None => serializer.serialize_none(),
    }
}
