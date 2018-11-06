pub mod bounds;
pub mod deserialization;
mod normalization;
mod validation;

use crate::dsl::object_types::bounds::IntegerObjectBounds;
use crate::dsl::object_types::bounds::StringObjectBounds;

#[derive(Clone, Debug)]
pub enum ObjectType {
    Required(RawObjectType),
    Optional(RawObjectType),
}

#[derive(Clone, Debug)]
pub enum RawObjectType {
    Object,
    Boolean,
    String(Option<StringObjectBounds>),
    Password(Option<StringObjectBounds>),
    Hostname,
    Integer(Option<IntegerObjectBounds>),
}

impl ObjectType {
    pub fn inner(&self) -> &RawObjectType {
        match self {
            ObjectType::Optional(object_type) => object_type,
            ObjectType::Required(object_type) => object_type,
        }
    }
}
