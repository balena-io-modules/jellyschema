use crate::dsl::enums::EnumerationValues;
use crate::dsl::schema::DisplayInformation;
use serde::de::Error;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde_yaml::Mapping;
use serde_yaml::Value;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct TypeInformation {
    pub spec: Option<TypeSpec>,
    pub enumeration_values: Option<EnumerationValues>,
}

impl<'de> Deserialize<'de> for TypeInformation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mapping = Mapping::deserialize(deserializer)?;

        let type_key = Value::from("type");
        let spec = &mapping.get(&type_key).map_or(Ok(None), |value| {
            serde_yaml::from_value(value.clone())
                .map_err(|e| Error::custom(format!("cannot deserialize type specifier: {:?} - {}", value, e)))
        })?;

        let enum_key = Value::from("enum");
        let enumeration_values: &Option<EnumerationValues> = &mapping.get(&enum_key).map_or(Ok(None), |value| {
            serde_yaml::from_value(value.clone()).map_err(|e| {
                Error::custom(format!(
                    "cannot deserialize list of enumeration values: {:?} - {}",
                    value, e
                ))
            })
        })?;

        Ok(TypeInformation {
            spec: spec.clone(),
            enumeration_values: enumeration_values.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct EnumerationValue {
    pub type_spec: TypeSpec,
    pub display_information: DisplayInformation,
    pub value: String
}

#[derive(Clone, Debug)]
pub enum ObjectType {
    Object,
    String,
    Hostname,
}

#[derive(Clone, Debug)]
pub enum TypeSpec {
    Required(ObjectType),
    Optional(ObjectType),
}

impl TypeSpec {
    pub fn inner(&self) -> &ObjectType {
        match self {
            TypeSpec::Optional(object_type) => object_type,
            TypeSpec::Required(object_type) => object_type,
        }
    }
}

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
                let type_spec = if type_name.ends_with('?') {
                    type_name.remove(type_name.len() - 1);
                    TypeSpec::Optional(ObjectType::from_str(&type_name)?)
                } else {
                    TypeSpec::Required(ObjectType::from_str(&type_name)?)
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
            "string" => ObjectType::String,
            "hostname" => ObjectType::Hostname,
            _ => return Err(Error::custom(format!("unknown object type {}", value))),
        };
        Ok(object_type)
    }
}
