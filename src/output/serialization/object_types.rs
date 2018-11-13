use std::string::ToString;

use heck::MixedCase;
use serde::ser::Error;
use serde::ser::SerializeMap;
use serde::Serialize;
use serde::Serializer;

use crate::dsl::schema::object_types::bounds::ArrayItemObjectBounds;
use crate::dsl::schema::object_types::bounds::ArrayObjectBounds;
use crate::dsl::schema::object_types::bounds::ArrayUniqueItemBound;
use crate::dsl::schema::object_types::bounds::BooleanObjectBounds;
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
    map.serialize_entry("type", object_type_name(raw_type))?;

    match raw_type {
        RawObjectType::Object => {}
        RawObjectType::Boolean(object_bounds) => serialize_boolean_with_bounds(object_bounds, map)?,
        RawObjectType::String(object_bounds) => serialize_string_with_bounds(object_bounds, map)?,
        RawObjectType::Password(object_bounds) => {
            map.serialize_entry("writeOnly", &true)?;
            serialize_string_with_bounds(object_bounds, map)?;
        }
        RawObjectType::Hostname => map.serialize_entry("format", "hostname")?,
        RawObjectType::Integer(object_bounds) => serialize_integer_bounds(object_bounds, map)?,
        RawObjectType::Array(object_bounds) => serialize_array_object_bounds(object_bounds, map)?,
    };
    Ok(())
}

pub fn object_type_name(object_type: &RawObjectType) -> &str {
    match object_type {
        RawObjectType::Object => "object",
        RawObjectType::Boolean(_) => "boolean",
        RawObjectType::String(_) => "string",
        RawObjectType::Password(_) => "string",
        RawObjectType::Hostname => "string",
        RawObjectType::Integer(_) => "integer",
        RawObjectType::Array(_) => "array",
    }
}

fn serialize_boolean_with_bounds<O, E, S>(bounds: &Option<BooleanObjectBounds>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
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

fn serialize_array_object_bounds<O, E, S>(bounds: &Option<ArrayObjectBounds>, map: &mut S) -> Result<(), E>
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
                ArrayItemObjectBounds::RespectiveItems(schemas) => {
                    map.serialize_entry("items", &schemas)?;
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
        if let Some(ref additional_items) = bounds.additional_items {
            map.serialize_entry("additionalItems", additional_items)?;
        }
    }
    Ok(())
}

fn serialize_integer_bounds<O, E, S>(bounds: &Option<IntegerObjectBounds>, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let Some(bounds) = bounds {
        serialize_integer_bound("maximum", &bounds.maximum, map)?;
        serialize_integer_bound("minimum", &bounds.minimum, map)?;

        if let Some(multiple_of) = bounds.multiple_of {
            map.serialize_entry("multipleOf", &multiple_of)?;
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
