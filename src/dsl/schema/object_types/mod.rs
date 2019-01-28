//! A `type` is anything that can be specified after `type` keyword as in the spec

use crate::dsl::schema::object_types::bounds::ArrayObjectBounds;
use crate::dsl::schema::object_types::bounds::DefaultValue;
use crate::dsl::schema::object_types::bounds::IntegerObjectBounds;
use crate::dsl::schema::object_types::bounds::StringObjectBounds;
use crate::dsl::schema::object_types::bounds::BooleanObjectBounds;

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
    /// to allow additional properties in the data validated against this schema or not ?
    additional_properties: bool,
}

#[derive(Clone, Debug)]
pub enum RawObjectType {
    Object,
    Boolean(Option<BooleanObjectBounds>),
    String(Option<StringObjectBounds>),
    Text(Option<StringObjectBounds>),
    Password(Option<StringObjectBounds>),
    Integer(Option<IntegerObjectBounds>),
    Number(Option<IntegerObjectBounds>),
    Array(Box<Option<ArrayObjectBounds>>),
    Stringlist(Box<Option<ArrayObjectBounds>>),

    File(Option<StringObjectBounds>),
    Port(Option<IntegerObjectBounds>),

    Datetime(Option<StringObjectBounds>),
    Date(Option<StringObjectBounds>),
    Time(Option<StringObjectBounds>),
    Hostname(Option<StringObjectBounds>),
    Email(Option<StringObjectBounds>),
    IPV4(Option<StringObjectBounds>),
    IPV6(Option<StringObjectBounds>),
    URI(Option<StringObjectBounds>),
    DnsmasqAddress(Option<StringObjectBounds>),
    ChronyAddress(Option<StringObjectBounds>),
    IpTablesAddress(Option<StringObjectBounds>),
}

impl ObjectTypeData {
    pub fn with_defaults(raw_type: RawObjectType) -> ObjectTypeData {
        ObjectTypeData {
            raw_type,
            default_value: None,
            additional_properties: false,
        }
    }

    pub fn new(
        raw_type: RawObjectType,
        default_value: Option<DefaultValue>,
        additional_properties: bool,
    ) -> ObjectTypeData {
        ObjectTypeData {
            raw_type,
            default_value,
            additional_properties,
        }
    }

    pub fn raw_type(&self) -> &RawObjectType {
        &self.raw_type
    }

    pub fn default_value(&self) -> &Option<DefaultValue> {
        &self.default_value
    }

    pub fn allow_additional_properties(&self) -> bool {
        self.additional_properties
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
