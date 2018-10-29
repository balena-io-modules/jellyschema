use serde::de::Error;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use std::fmt;
use std::fmt::Formatter;

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
