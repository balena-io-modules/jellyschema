use crate::dsl::object_types::bounds::EnumerationValue;
use crate::dsl::object_types::bounds::IntegerBound;
use crate::dsl::object_types::bounds::IntegerObjectBounds;
use crate::dsl::object_types::bounds::StringLength;
use crate::dsl::object_types::bounds::StringObjectBounds;
use crate::dsl::object_types::RawObjectType;
use serde::ser::Error;
use serde::ser::SerializeMap;
use serde::Serialize;
use serde::Serializer;
use std::string::ToString;

use heck::MixedCase;
use crate::dsl::object_types::bounds::BooleanObjectBounds;

impl Serialize for EnumerationValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        if self.display_information.title.is_some() {
            map.serialize_entry("title", &self.display_information.title)?;
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
    match raw_type {
        RawObjectType::Object => map.serialize_entry("type", "object")?,
        RawObjectType::Boolean(object_bounds) => {
            serialize_boolean_with_bounds(object_bounds, map)?;
        },
        RawObjectType::String(object_bounds) => {
            serialize_string_with_bounds(object_bounds, map)?;
        }
        RawObjectType::Password(object_bounds) => {
            map.serialize_entry("writeOnly", &true)?;
            serialize_string_with_bounds(object_bounds, map)?;
        }
        RawObjectType::Hostname => {
            map.serialize_entry("type", "string")?;
            map.serialize_entry("format", "hostname")?
        }
        RawObjectType::Integer(object_bounds) => {
            map.serialize_entry("type", "integer")?;
            for bounds in object_bounds {
                serialize_integer_bounds(bounds, map)?;
            }
        }
    };
    Ok(())
}

fn serialize_boolean_with_bounds<O, E, S>(bounds: &Option<BooleanObjectBounds>, map: &mut S) -> Result<(), E>
    where
        E: Error,
        S: SerializeMap<Ok = O, Error = E>,
{
    map.serialize_entry("type", "boolean")?;
    for value in bounds {
        match value {
            BooleanObjectBounds::DefaultValue(default_value) => {
                map.serialize_entry("default", default_value)?;
            }
        }
    }
    Ok(())
}

fn serialize_string_with_bounds<O, E, S>(bounds: &Option<StringObjectBounds>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    map.serialize_entry("type", "string")?;
    for enumeration_values in bounds {
        serialize_string_bounds(&enumeration_values, map)?;
    }
    Ok(())
}

fn serialize_integer_bound<O, E, S>(name: &str, bound: &Option<IntegerBound>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if bound.is_some() {
        let value = bound.unwrap();
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

fn serialize_integer_bounds<O, E, S>(bounds: &IntegerObjectBounds, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    serialize_integer_bound("maximum", &bounds.maximum, map)?;
    serialize_integer_bound("minimum", &bounds.minimum, map)?;

    if bounds.multiple_of.is_some() {
        map.serialize_entry("multipleOf", &bounds.multiple_of.unwrap())?;
    }
    Ok(())
}

fn serialize_string_bounds<O, E, S>(string_bounds: &StringObjectBounds, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    match string_bounds {
        StringObjectBounds::PossibleValues(values) => {
            if values.len() == 1 {
                serialize_singular_constant_value(&values[0], map)?;
            } else {
                serialize_multiple_enum_values(&values, map)?;
            }
        }
        StringObjectBounds::Pattern(pattern) => map.serialize_entry("pattern", pattern.as_str())?,
        StringObjectBounds::Length(length) => serialize_length_bounds(length, map)?,
    }
    Ok(())
}

fn serialize_length_bounds<O, E, S>(length_bounds: &StringLength, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if length_bounds.maximum.is_some() {
        map.serialize_entry("maxLength", &length_bounds.maximum.unwrap())?;
    }
    if length_bounds.minimum.is_some() {
        map.serialize_entry("minLength", &length_bounds.minimum.unwrap())?;
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
