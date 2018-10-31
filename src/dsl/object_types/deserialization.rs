use crate::dsl::enums::EnumerationValues;
use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::RawObjectType;
use crate::dsl::object_types::TypeDefinition;
use serde::de::Error;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde_yaml::Mapping;
use serde_yaml::Value;
use std::fmt;
use std::fmt::Formatter;

impl<'de> Deserialize<'de> for TypeDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mapping = Mapping::deserialize(deserializer)?;

        let type_key = Value::from("type");
        let spec = &mapping.get(&type_key).map_or(Ok(None), |value| {
            serde_yaml::from_value(value.clone()).map_err(|e| Error::custom(format!("{}", e)))
        })?;

        let enum_key = Value::from("enum");
        let enumeration_values: &Option<EnumerationValues> = &mapping.get(&enum_key).map_or(Ok(None), |value| {
            serde_yaml::from_value(value.clone()).map_err(|e| {
                Error::custom(format!(
                    "cannot deserialize list of enumeration values: {:#?} - {}",
                    value, e
                ))
            })
        })?;

        let constant_key = Value::from("const");
        let constant = &mapping.get(&constant_key).map_or(Ok(None), |value| {
            serde_yaml::from_value(value.clone())
                .map_err(|e| Error::custom(format!("cannot deserialize constant specifier: {:?} - {}", value, e)))
        })?;

        Ok(TypeDefinition {
            r#type: spec.clone(),
            enumeration_values: enumeration_values.clone(),
            constant_value: constant.clone(),
        })
    }
}

impl<'de> Deserialize<'de> for ObjectType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TypeSpecVisitor;

        impl<'de> Visitor<'de> for TypeSpecVisitor {
            type Value = ObjectType;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("type name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let mut type_name = value.trim().to_lowercase();
                let type_spec = if type_name.ends_with('?') {
                    type_name.remove(type_name.len() - 1);
                    ObjectType::Optional(RawObjectType::parse(&type_name)?)
                } else {
                    ObjectType::Required(RawObjectType::parse(&type_name)?)
                };
                Ok(type_spec)
            }
        }

        deserializer.deserialize_any(TypeSpecVisitor)
    }
}

impl RawObjectType {
    fn parse<E>(value: &str) -> Result<Self, E>
    where
        E: Error,
    {
        let object_type = match value {
            "object" => RawObjectType::Object,
            "string" => RawObjectType::String,
            "hostname" => RawObjectType::Hostname,
            "integer" => RawObjectType::Integer,
            _ => return Err(Error::custom(format!("unknown object type `{}`", value))),
        };
        Ok(object_type)
    }
}
