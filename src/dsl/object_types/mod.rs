pub mod bounds;
pub mod deserialization;
mod normalization;
mod validation;

use crate::dsl::object_types::bounds::ArrayObjectBounds;
use crate::dsl::object_types::bounds::BooleanObjectBounds;
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
    Boolean(Option<BooleanObjectBounds>),
    String(Option<StringObjectBounds>),
    Password(Option<StringObjectBounds>),
    Hostname,
    Integer(Option<IntegerObjectBounds>),
    Array(Option<ArrayObjectBounds>),
}

impl RawObjectType {
    pub fn has_bounds(&self) -> bool {
        match self {
            RawObjectType::Object => false,
            RawObjectType::Hostname => false,
            RawObjectType::Boolean(bounds) => bounds.is_some(),
            RawObjectType::String(bounds) => bounds.is_some(),
            RawObjectType::Password(bounds) => bounds.is_some(),
            RawObjectType::Integer(bounds) => bounds.is_some(),
            RawObjectType::Array(bounds) => bounds.is_some(),
        }
    }
}

impl ObjectType {
    pub fn inner(&self) -> &RawObjectType {
        match self {
            ObjectType::Optional(object_type) => object_type,
            ObjectType::Required(object_type) => object_type,
        }
    }
}
