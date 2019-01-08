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
    default_value: Option<DefaultValue>,
}

#[derive(Clone, Debug)]
pub enum RawObjectType {
    Object,
    Boolean,
    String(Option<StringObjectBounds>),
    Text(Option<StringObjectBounds>),
    Password(Option<StringObjectBounds>),
    Integer(Option<IntegerObjectBounds>),
    Number(Option<IntegerObjectBounds>),
    Array(Box<Option<ArrayObjectBounds>>),

    Binary,
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

impl ObjectTypeData {
    pub fn with_raw_type(raw_type: RawObjectType) -> ObjectTypeData {
        ObjectTypeData {
            raw_type,
            default_value: None,
        }
    }

    pub fn with_raw_type_and_default_value(
        raw_type: RawObjectType,
        default_value: Option<DefaultValue>,
    ) -> ObjectTypeData {
        ObjectTypeData {
            raw_type,
            default_value,
        }
    }

    pub fn raw_type(&self) -> &RawObjectType {
        &self.raw_type
    }

    pub fn default_value(&self) -> &Option<DefaultValue> {
        &self.default_value
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
            RawObjectType::Binary => false,
            RawObjectType::Boolean => false,
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
    pub fn data(&self) -> &ObjectTypeData {
        match self {
            ObjectType::Optional(object_type_data) => &object_type_data,
            ObjectType::Required(object_type_data) => &object_type_data,
        }
    }

    pub fn inner_raw(&self) -> &RawObjectType {
        match self {
            ObjectType::Optional(object_type_data) => &object_type_data.raw_type,
            ObjectType::Required(object_type_data) => &object_type_data.raw_type,
        }
    }
}
