use crate::dsl::object_types::bounds::EnumerationValue;
use crate::dsl::object_types::bounds::IntegerBound;
use crate::dsl::object_types::bounds::IntegerObjectBounds;
use crate::dsl::object_types::bounds::StringLength;
use crate::dsl::object_types::bounds::StringObjectBounds;
use crate::dsl::object_types::{ObjectType, RawObjectType};
use crate::dsl::schema::{Property, PropertyList};
use heck::MixedCase;
use serde::ser::{Error, Serialize, SerializeMap, Serializer};
use std::collections::HashMap;

pub fn serialize_property_list<O, E, S>(property_list: &PropertyList, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if !property_list.entries.is_empty() {
        let mut properties_map = HashMap::new();
        for property in &property_list.entries {
            properties_map.insert(&property.name, &property.property);
        }
        map.serialize_entry("properties", &properties_map)?;
    };

    if !property_list.required_property_names().is_empty() {
        map.serialize_entry("required", &property_list.required_property_names())?;
    }
    if !property_list.property_names().is_empty() {
        map.serialize_entry("$$order", &property_list.property_names())?;
    }
    Ok(())
}

impl Serialize for Property {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        for title in &self.display_information.title {
            map.serialize_entry("title", &title)?;
        }

        for type_spec in &self.type_information {
            serialize_object_type(&type_spec.inner(), &mut map)?;
        }

        let property_list = &self.property_list;
        if property_list.is_some() {
            serialize_property_list(&property_list.clone().unwrap(), &mut map)?;
        }

        map.end()
    }
}

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

impl Serialize for ObjectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        serialize_object_type(&self.inner(), &mut map)?;
        map.end()
    }
}

fn serialize_object_type<O, E, S>(raw_type: &RawObjectType, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    match raw_type {
        RawObjectType::Object => map.serialize_entry("type", "object")?,
        RawObjectType::String(object_bounds) => {
            map.serialize_entry("type", "string")?;
            for enumeration_values in object_bounds {
                serialize_string_bounds(&enumeration_values, map)?;
            }
        }
        RawObjectType::Password(object_bounds) => {
            map.serialize_entry("type", "string")?;
            map.serialize_entry("writeOnly", &true)?;
            for enumeration_values in object_bounds {
                serialize_string_bounds(&enumeration_values, map)?;
            }
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
