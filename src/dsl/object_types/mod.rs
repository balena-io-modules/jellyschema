mod deserialization;
mod normalization;
mod validation;

use crate::dsl::enums::EnumerationValues;

#[derive(Clone, Debug)]
pub struct TypeDefinition {
    pub r#type: Option<ObjectType>,
    pub enumeration_values: Option<EnumerationValues>,
    pub constant_value: Option<String>,
}

#[derive(Clone, Debug)]
pub enum ObjectType {
    Required(RawObjectType),
    Optional(RawObjectType),
}

#[derive(Clone, Debug)]
pub enum RawObjectType {
    Object,
    String(Option<EnumerationValues>),
    Hostname,
    Integer(Vec<IntegerObjectBound>),
}

#[derive(Clone, Debug)]
pub enum IntegerObjectBound {
    Minimum(u64),
    Maximum(u64),
}

impl ObjectType {
    pub fn inner(&self) -> &RawObjectType {
        match self {
            ObjectType::Optional(object_type) => object_type,
            ObjectType::Required(object_type) => object_type,
        }
    }
}
