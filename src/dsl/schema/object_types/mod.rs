//! A `type` is anything that can be specified after `type` keyword as in the spec

use crate::dsl::schema::object_types::bounds::ArrayObjectBounds;
use crate::dsl::schema::object_types::bounds::DefaultValue;
use crate::dsl::schema::object_types::bounds::IntegerObjectBounds;
use crate::dsl::schema::object_types::bounds::StringObjectBounds;

pub mod bounds;
pub mod deserialization;

#[derive(Clone, Debug)]
pub enum ObjectType {
    Required(ObjectTypeData),
    Optional(ObjectTypeData),
}

#[derive(Clone, Debug)]
pub struct ObjectTypeData {
   raw_type: RawObjectType,
}

#[derive(Clone, Debug)]
pub enum RawObjectType {
    Object,
    Boolean(Option<DefaultValue>),
    String(Option<StringObjectBounds>),
    Text(Option<StringObjectBounds>),
    Password(Option<StringObjectBounds>),
    Integer(Option<IntegerObjectBounds>, Option<DefaultValue>),
    Number(Option<IntegerObjectBounds>, Option<DefaultValue>),
    Array(Box<Option<ArrayObjectBounds>>),

    Port(Option<IntegerObjectBounds>, Option<DefaultValue>),

    Datetime,
    Date,
    Time,
    Hostname,
    Email,
    IPV4,
    IPV6,
    URI,
}

impl ObjectTypeData {
    pub fn with_raw_type(raw_type: RawObjectType) -> ObjectTypeData {
        ObjectTypeData{raw_type}
    }

    pub fn raw_type(&self) -> &RawObjectType {
        &self.raw_type
    }
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
            RawObjectType::Integer(bounds, _) => bounds.is_some(),
            RawObjectType::Number(bounds, _) => bounds.is_some(),
            RawObjectType::Port(bounds, _) => bounds.is_some(),
            RawObjectType::Array(bounds) => bounds.is_some(),
        }
    }
}

impl ObjectType {
    pub fn inner(&self) -> &RawObjectType {
        match self {
            ObjectType::Optional(object_type_data) => &object_type_data.raw_type,
            ObjectType::Required(object_type_data) => &object_type_data.raw_type,
        }
    }
}
