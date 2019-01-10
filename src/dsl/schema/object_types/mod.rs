//! A `type` is anything that can be specified after `type` keyword as in the spec

use crate::dsl::schema::object_types::bounds::ArrayObjectBounds;
use crate::dsl::schema::object_types::bounds::DefaultValue;
use crate::dsl::schema::object_types::bounds::IntegerObjectBounds;
use crate::dsl::schema::object_types::bounds::StringObjectBounds;

pub mod bounds;
pub mod deserialization;

#[derive(Clone, Debug)]
pub enum ObjectType {
    Required(RawObjectType),
    Optional(RawObjectType),
}

#[derive(Clone, Debug)]
pub enum RawObjectType {
    Object,
    Boolean(Option<DefaultValue<bool>>),
    String(Option<StringObjectBounds>),
    Text(Option<StringObjectBounds>),
    Password(Option<StringObjectBounds>),
    Integer(Option<IntegerObjectBounds>),
    Number(Option<IntegerObjectBounds>),
    Array(Box<Option<ArrayObjectBounds>>),

    Port(Option<IntegerObjectBounds>),

    Datetime,
    Date,
    Time,
    Hostname,
    Email,
    IPV4,
    IPV6,
    URI,
}

impl RawObjectType {
    pub fn has_bounds(&self) -> bool {
        match self {
            RawObjectType::Object => false,
            RawObjectType::Hostname => false,
            RawObjectType::Datetime => false,
            RawObjectType::Date => false,
            RawObjectType::Time => false,
            RawObjectType::Email => false,
            RawObjectType::IPV4 => false,
            RawObjectType::IPV6 => false,
            RawObjectType::URI => false,
            RawObjectType::Boolean(bounds) => bounds.is_some(),
            RawObjectType::String(bounds) => bounds.is_some(),
            RawObjectType::Text(bounds) => bounds.is_some(),
            RawObjectType::Password(bounds) => bounds.is_some(),
            RawObjectType::Integer(bounds) => bounds.is_some(),
            RawObjectType::Number(bounds) => bounds.is_some(),
            RawObjectType::Port(bounds) => bounds.is_some(),
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
