use crate::dsl::compiler::ObjectType;
use crate::dsl::compiler::Property;
use crate::dsl::compiler::PropertyEntry;
use crate::dsl::compiler::PropertyList;
use crate::dsl::compiler::TypeSpec;
use serde::de::Error;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use serde_yaml::Mapping;
use serde_yaml::Sequence;
use std::fmt;
use std::fmt::Formatter;

impl<'de> Deserialize<'de> for TypeSpec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TypeSpecVisitor;

        impl<'de> Visitor<'de> for TypeSpecVisitor {
            type Value = TypeSpec;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("type name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let mut type_name = value.trim().to_lowercase();
                let type_spec = match type_name.ends_with('?') {
                    true => {
                        type_name.remove(type_name.len() - 1);
                        TypeSpec::Optional(ObjectType::from_str(&type_name)?)
                    }
                    false => TypeSpec::Required(ObjectType::from_str(&type_name)?),
                };
                Ok(type_spec)
            }
        }

        deserializer.deserialize_any(TypeSpecVisitor)
    }
}

impl ObjectType {
    fn from_str<E>(value: &str) -> Result<Self, E>
    where
        E: Error,
    {
        let object_type = match value {
            "object" => ObjectType::Object,
            "hostname" => ObjectType::Hostname,
            _ => return Err(Error::custom(format!("unknown object type {}", value))),
        };
        Ok(object_type)
    }
}

pub fn deserialize_property_list<'de, D>(deserializer: D) -> Result<Option<PropertyList>, D::Error>
where
    D: Deserializer<'de>,
{
    let maybe_sequence: Option<serde_yaml::Sequence> = Option::deserialize(deserializer)?;
    match maybe_sequence {
        Some(sequence) => Ok(Some(sequence_to_property_list(sequence)?)),
        None => Ok(None),
    }
}

fn sequence_to_property_list<E>(sequence: Sequence) -> Result<PropertyList, E>
where
    E: Error,
{
    let mut property_names = vec![];
    let list_of_maybe_entries = sequence.into_iter().map(|value| {
        let mapping = value
            .as_mapping()
            .ok_or(Error::custom("cannot deserialize property as mapping"))?;
        let property_entry = mapping_to_property_entry(mapping)?;
        property_names.push(property_entry.name.clone());
        Ok(property_entry)
    });

    let list: Result<Vec<_>, E> = list_of_maybe_entries.collect();
    let list = list?;

    Ok(PropertyList {
        entries: list,
        property_names: property_names.clone(),
    })
}

fn mapping_to_property_entry<E>(mapping: &Mapping) -> Result<PropertyEntry, E>
where
    E: Error,
{
    let (key, value) = mapping
        .into_iter()
        .next()
        .ok_or(Error::custom("cannot get first element of the sequence"))?;
    let key: String = serde_yaml::from_value(key.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize property key - {}", e)))?;
    let value: Property = serde_yaml::from_value(value.clone())
        .map_err(|e| Error::custom(format!("cannot deserialize property value - {}", e)))?;
    Ok(PropertyEntry {
        name: key,
        property: value,
    })
}
