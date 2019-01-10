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

pub fn serialize_object_type<O, E, S>(raw_type: &RawObjectType, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    map.serialize_entry("type", object_type_name(raw_type))?;

    match raw_type {
        RawObjectType::Object => {}
        RawObjectType::Boolean(default_value) => serialize_boolean(default_value, map)?,
        RawObjectType::String(object_bounds) => serialize_string(object_bounds, map)?,
        RawObjectType::Text(object_bounds) => serialize_string(object_bounds, map)?,
        RawObjectType::Password(object_bounds) => serialize_password(object_bounds, map)?,
        RawObjectType::Integer(object_bounds, default) => serialize_integer(object_bounds, default, map)?,
        RawObjectType::Number(object_bounds, default) => serialize_integer(object_bounds, default, map)?,
        RawObjectType::Port(object_bounds, default) => serialize_integer(object_bounds, default, map)?,
        RawObjectType::Array(object_bounds) => serialize_array(object_bounds, map)?,

        RawObjectType::Hostname => map.serialize_entry("format", "hostname")?,
        RawObjectType::Datetime => map.serialize_entry("format", "date-time")?,
        RawObjectType::Date => map.serialize_entry("format", "date")?,
        RawObjectType::Time => map.serialize_entry("format", "time")?,
        RawObjectType::Email => map.serialize_entry("format", "email")?,
        RawObjectType::IPV4 => map.serialize_entry("format", "ipv4")?,
        RawObjectType::IPV6 => map.serialize_entry("format", "ipv6")?,
        RawObjectType::URI => map.serialize_entry("format", "uri")?,
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
        RawObjectType::Integer(_, _) => "integer",
        RawObjectType::Number(_, _) => "number",
        RawObjectType::Port(_, _) => "number",
        RawObjectType::Array(_) => "array",

        RawObjectType::Hostname => "string",
        RawObjectType::Datetime => "string",
        RawObjectType::Date => "string",
        RawObjectType::Time => "string",
        RawObjectType::Email => "string",
        RawObjectType::IPV4 => "string",
        RawObjectType::IPV6 => "string",
        RawObjectType::URI => "string",
    }
}

fn serialize_boolean<O, E, S>(default: &Option<DefaultValue>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    serialize_default(default, map)
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

fn serialize_integer<O, E, S>(bounds: &Option<IntegerObjectBounds>, default: &Option<DefaultValue>, map: &mut S) -> Result<(), E>
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
            IntegerObjectBounds::List(list) => serialize_enum_bounds(list, map)?,
        }
    }
    serialize_default(default, map)?;
    Ok(())
}

fn serialize_string_bounds<O, E, S>(string_bounds: &StringObjectBounds, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    match string_bounds {
        StringObjectBounds::List(values) => serialize_enum_bounds(values, map)?,
        StringObjectBounds::Pattern(pattern) => map.serialize_entry("pattern", pattern.as_str())?,
        StringObjectBounds::Length(length) => serialize_length_bounds(length, map)?,
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
