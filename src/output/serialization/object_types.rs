use std::collections::HashMap;
use std::string::ToString;

use heck::MixedCase;
use serde::ser::Error;
use serde::ser::SerializeMap;
use serde::Serialize;
use serde::Serializer;

use crate::dsl::schema::object_types::bounds::ArrayItemObjectBounds;
use crate::dsl::schema::object_types::bounds::ArrayObjectBounds;
use crate::dsl::schema::object_types::bounds::ArrayUniqueItemBound;
use crate::dsl::schema::object_types::bounds::DefaultValue;
use crate::dsl::schema::object_types::bounds::EnumerationValue;
use crate::dsl::schema::object_types::bounds::IntegerBound;
use crate::dsl::schema::object_types::bounds::IntegerObjectBounds;
use crate::dsl::schema::object_types::bounds::StringLength;
use crate::dsl::schema::object_types::bounds::StringObjectBounds;
use crate::dsl::schema::object_types::RawObjectType;
use crate::dsl::schema::object_types::ObjectType;
use crate::dsl::schema::object_types::bounds::BooleanObjectBounds;

impl Serialize for EnumerationValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        if self.annotations.title.is_some() {
            map.serialize_entry("title", &self.annotations.title)?;
            map.serialize_entry("enum", &vec![&self.value])?;
        }

        map.end()
    }
}

pub fn serialize_object_type<O, E, S>(object_type: &ObjectType, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    let raw_type = object_type.inner_raw();
    let default = object_type.data().default_value();

    map.serialize_entry("type", object_type_name(raw_type))?;
    serialize_default(default, map)?;

    match raw_type {
        RawObjectType::Object => {
            map.serialize_entry("additionalProperties", &false)?;
        }
        RawObjectType::Boolean(object_bounds) => serialize_boolean(object_bounds, map)?,
        RawObjectType::String(object_bounds) => serialize_string(object_bounds, map)?,
        RawObjectType::Text(object_bounds) => serialize_string(object_bounds, map)?,
        RawObjectType::Password(object_bounds) => serialize_password(object_bounds, map)?,
        RawObjectType::Integer(object_bounds) => serialize_integer(object_bounds, map)?,
        RawObjectType::Number(object_bounds) => serialize_integer(object_bounds, map)?,
        RawObjectType::Port(object_bounds) => serialize_integer(object_bounds, map)?,
        RawObjectType::Array(object_bounds) => serialize_array(object_bounds, map)?,
        RawObjectType::Stringlist(object_bounds) => serialize_array(object_bounds, map)?,

        RawObjectType::Hostname(object_bounds) => serialize_string_with_format("hostname", object_bounds, map)?,
        RawObjectType::Datetime(object_bounds) => serialize_string_with_format("date-time", object_bounds, map)?,
        RawObjectType::Date(object_bounds) => serialize_string_with_format("date", object_bounds, map)?,
        RawObjectType::Time(object_bounds) => serialize_string_with_format("time", object_bounds, map)?,
        RawObjectType::Email(object_bounds) => serialize_string_with_format("email", object_bounds, map)?,
        RawObjectType::IPV4(object_bounds) => serialize_string_with_format("ipv4", object_bounds, map)?,
        RawObjectType::IPV6(object_bounds) => serialize_string_with_format("ipv6", object_bounds, map)?,
        RawObjectType::URI(object_bounds) => serialize_string_with_format("uri", object_bounds, map)?,
        RawObjectType::File(object_bounds) => serialize_string_with_format("data-url", object_bounds, map)?,

        RawObjectType::DnsmasqAddress(object_bounds) => {
            serialize_string_with_format("dnsmasq-address", object_bounds, map)?
        }
        RawObjectType::ChronyAddress(object_bounds) => {
            serialize_string_with_format("chrony-address", object_bounds, map)?
        }
        RawObjectType::IpTablesAddress(object_bounds) => {
            serialize_string_with_format("iptables-address", object_bounds, map)?
        }
    };
    Ok(())
}

pub fn object_type_name(object_type: &RawObjectType) -> &str {
    match object_type {
        RawObjectType::Object => "object",
        RawObjectType::Boolean(_) => "boolean",
        RawObjectType::String(_) => "string",
        RawObjectType::Text(_) => "string",
        RawObjectType::Password(_) => "string",
        RawObjectType::Integer(_) => "integer",
        RawObjectType::Number(_) => "number",
        RawObjectType::Port(_) => "integer",
        RawObjectType::Array(_) => "array",
        RawObjectType::Stringlist(_) => "array",

        RawObjectType::Hostname(_) => "string",
        RawObjectType::Datetime(_) => "string",
        RawObjectType::Date(_) => "string",
        RawObjectType::Time(_) => "string",
        RawObjectType::Email(_) => "string",
        RawObjectType::IPV4(_) => "string",
        RawObjectType::IPV6(_) => "string",
        RawObjectType::URI(_) => "string",

        RawObjectType::File(_) => "string",

        RawObjectType::DnsmasqAddress(_) => "string",
        RawObjectType::ChronyAddress(_) => "string",
        RawObjectType::IpTablesAddress(_) => "string",
    }
}

fn serialize_default<O, E, S>(default: &Option<DefaultValue>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    for value in default {
        map.serialize_entry("default", value.value())?;
    }
    Ok(())
}

fn serialize_boolean<O, E, S>(bounds: &Option<BooleanObjectBounds>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let Some(bounds) = bounds {
        serialize_enum_bounds(&bounds.0, map)?;
    }
    Ok(())
}

fn serialize_string<O, E, S>(bounds: &Option<StringObjectBounds>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    for enumeration_values in bounds {
        serialize_string_bounds(&enumeration_values, map)?;
    }
    Ok(())
}

fn serialize_string_with_format<O, E, S>(
    format: &str,
    bounds: &Option<StringObjectBounds>,
    map: &mut S,
) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    map.serialize_entry("format", format)?;
    serialize_string(bounds, map)
}

fn serialize_password<O, E, S>(bounds: &Option<StringObjectBounds>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    map.serialize_entry("writeOnly", &true)?;
    serialize_string(bounds, map)?;
    Ok(())
}

fn serialize_integer_bound<O, E, S>(name: &str, bound: &Option<IntegerBound>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let Some(value) = bound {
        match value {
            IntegerBound::Inclusive(value) => map.serialize_entry(name, &value)?,
            IntegerBound::Exclusive(value) => {
                map.serialize_entry(name, &value)?;
                map.serialize_entry(&("exclusive ".to_string() + name).to_mixed_case(), &true)?;
            }
        }
    }
    Ok(())
}

fn serialize_array<O, E, S>(bounds: &Option<ArrayObjectBounds>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let Some(bounds) = bounds {
        if let Some(max) = bounds.maximum_number_of_items {
            map.serialize_entry("maxItems", &max)?;
        }
        if let Some(min) = bounds.minimum_number_of_items {
            map.serialize_entry("minItems", &min)?;
        }
        if let Some(ref items) = bounds.items {
            match items {
                ArrayItemObjectBounds::AllItems(schema) => {
                    map.serialize_entry("items", &schema)?;
                }
                ArrayItemObjectBounds::AnyItems(schemas) => {
                    let mut wrapper = HashMap::new();
                    wrapper.insert("oneOf", schemas);
                    map.serialize_entry("items", &wrapper)?;
                }
            }
        }
        if let Some(ref unique_items) = bounds.unique_items {
            match unique_items {
                ArrayUniqueItemBound::All => {
                    map.serialize_entry("uniqueItems", &true)?;
                }
                ArrayUniqueItemBound::Specific(items) => {
                    map.serialize_entry("$$uniqueItemProperties", items)?;
                }
            }
        }
    }
    Ok(())
}

fn serialize_integer<O, E, S>(bounds: &Option<IntegerObjectBounds>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let Some(bounds) = bounds {
        match bounds {
            IntegerObjectBounds::Conditions(conditions) => {
                serialize_integer_bound("maximum", &conditions.maximum, map)?;
                serialize_integer_bound("minimum", &conditions.minimum, map)?;
                if let Some(multiple_of) = conditions.multiple_of {
                    map.serialize_entry("multipleOf", &multiple_of)?;
                }
            }
            IntegerObjectBounds::Enumeration(list) => serialize_enum_bounds(list, map)?,
        }
    }
    Ok(())
}

fn serialize_string_bounds<O, E, S>(string_bounds: &StringObjectBounds, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    match string_bounds {
        StringObjectBounds::Enumeration(values) => serialize_enum_bounds(values, map)?,
        StringObjectBounds::Value(value_bounds) => {
            if let Some(pattern) = &value_bounds.pattern {
                map.serialize_entry("pattern", pattern.as_str())?
            }
            if let Some(length) = &value_bounds.length {
                serialize_length_bounds(length, map)?
            }
        }
    }
    Ok(())
}

fn serialize_enum_bounds<O, E, S>(values: &[EnumerationValue], map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if values.len() == 1 {
        serialize_singular_constant_value(&values[0], map)?;
    } else {
        serialize_multiple_enum_values(&values, map)?;
    }
    Ok(())
}

fn serialize_length_bounds<O, E, S>(length_bounds: &StringLength, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let Some(maximum) = length_bounds.maximum {
        map.serialize_entry("maxLength", &maximum)?;
    }
    if let Some(minimum) = length_bounds.minimum {
        map.serialize_entry("minLength", &minimum)?;
    }
    Ok(())
}

fn serialize_multiple_enum_values<O, E, S>(enumeration_values: &[EnumerationValue], map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if !enumeration_values.is_empty() {
        map.serialize_entry("oneOf", &enumeration_values)?;
    }
    Ok(())
}

fn serialize_singular_constant_value<O, E, S>(constant: &EnumerationValue, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    Ok(map.serialize_entry("enum", &vec![constant.value.clone()]))?
}
