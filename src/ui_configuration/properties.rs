use crate::dsl::enums::EnumerationValue;
use crate::dsl::enums::EnumerationValues;
use crate::dsl::object_types::IntegerBound;
use crate::dsl::object_types::IntegerObjectBounds;
use crate::dsl::object_types::{ObjectType, RawObjectType};
use crate::dsl::schema::{Property, PropertyList};
use heck::MixedCase;
use serde::ser::{Error, Serialize, SerializeMap, Serializer};

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

impl Serialize for PropertyList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries_count = self.entries.iter().count();
        let mut map = serializer.serialize_map(Some(entries_count))?;
        for entry in &self.entries {
            map.serialize_entry(&entry.name, &entry.property)?;
        }
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
                serialize_enumeration_values(&enumeration_values, map)?;
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

fn serialize_enumeration_values<O, E, S>(enumeration_values: &EnumerationValues, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    let enumeration_possible_values = &enumeration_values.possible_values;

    if enumeration_possible_values.iter().count() == 1 {
        serialize_singular_constant_value(enumeration_possible_values.iter().next().unwrap(), map)?;
    } else {
        serialize_multiple_enum_values(enumeration_possible_values, map)?;
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
