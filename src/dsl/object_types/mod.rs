mod deserialization;

use crate::dsl::enums::EnumerationValues;
use serde::de::Error;

#[derive(Clone, Debug)]
pub struct TypeDefinition {
    pub r#type: Option<ObjectType>,
    pub enumeration_values: Option<EnumerationValues>,
}

#[derive(Clone, Debug)]
pub enum ObjectType {
    Required(RawObjectType),
    Optional(RawObjectType),
}

#[derive(Clone, Debug)]
pub enum RawObjectType {
    Object,
    String,
    Hostname,
}

impl ObjectType {
    pub fn inner(&self) -> &RawObjectType {
        match self {
            ObjectType::Optional(object_type) => object_type,
            ObjectType::Required(object_type) => object_type,
        }
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
            _ => return Err(Error::custom(format!("unknown object type {}", value))),
        };
        Ok(object_type)
    }
}
