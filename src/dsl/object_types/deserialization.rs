use crate::dsl::enums::deserialization::deserialize_enumeration_values;
use crate::dsl::enums::EnumerationValues;
use crate::dsl::object_types::IntegerBound;
use crate::dsl::object_types::IntegerObjectBounds;
use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::RawObjectType;
use heck::MixedCase;
use serde::de::Error;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde_yaml::Mapping;
use serde_yaml::Value;
use std::fmt;
use std::fmt::Formatter;

pub fn deserialize_object_type<E>(mapping: &Mapping) -> Result<Option<ObjectType>, E>
where
    E: Error,
{
    let type_key = Value::from("type");
    Ok(mapping.get(&type_key).map_or(Ok(None), |value| match value.as_str() {
        Some(value) => Ok(Some({
            let mut type_name = value.trim().to_lowercase();
            if type_name.ends_with('?') {
                type_name.remove(type_name.len() - 1);
                ObjectType::Optional(RawObjectType::from(&type_name, &mapping)?)
            } else {
                ObjectType::Required(RawObjectType::from(&type_name, &mapping)?)
            }
        })),
        None => Err(Error::custom("cannot find type definition")),
    })?)
}

fn deserialize_integer<E>(name: &str, mapping: &Mapping) -> Result<Option<i64>, E>
where
    E: Error,
{
    let maximum_key = Value::from(name);
    let value = mapping.get(&maximum_key);
    println!("value from mapping {:#?} for {}", &value, &name);

    match value {
        None => Ok(None),
        Some(value) => match value.as_i64() {
            None => Err(Error::custom(format!("cannot deserialize {:#?} as integer", value))),
            Some(value) => Ok(Some(value)),
        },
    }
}

fn deserialize_minmax<E>(name: &str, mapping: &Mapping) -> Result<Option<IntegerBound>, E>
where
    E: Error,
{
    let normal = deserialize_integer(name, mapping)?;
    let exclusive = deserialize_integer(&("exclusive ".to_string() + name).to_mixed_case(), mapping)?;
    if normal.is_some() && exclusive.is_some() {
        return Err(Error::custom("cannot have both {} and exclusive {} set"));
    }
    if normal.is_some() {
        return Ok(Some(IntegerBound::Inclusive(normal.unwrap())));
    }
    if exclusive.is_some() {
        return Ok(Some(IntegerBound::Exclusive(exclusive.unwrap())));
    }
    Ok(None)
}

fn deserialize_integer_bounds<E>(mapping: &Mapping) -> Result<Option<IntegerObjectBounds>, E>
where
    E: Error,
{
    let minimum_key = Value::from("minimum");
    let multiple_of_key = Value::from("multipleOf");
    let exclusive_maximum_key = Value::from("exclusiveMaximum");
    let maximum = deserialize_minmax("maximum", mapping)?;
    let minimum = deserialize_minmax("minimum", mapping)?;
    let multiple_of = deserialize_integer("multipleOf", mapping)?;
    println!("maximum: {:#?}", &maximum);
    println!("minimum: {:#?}", &minimum);

    if maximum.is_some() {
        Ok(Some(IntegerObjectBounds {
            minimum,
            maximum,
            multiple_of,
        }))
    } else {
        Ok(None)
    }
}

impl RawObjectType {
    fn from<E>(value: &str, mapping: &Mapping) -> Result<Self, E>
    where
        E: Error,
    {
        let object_type = match value {
            "object" => RawObjectType::Object, // fixme - deserialize recursively here
            "string" => {
                let enumeration_values = deserialize_enumeration_values(mapping)?;
                RawObjectType::String(enumeration_values)
            }
            "hostname" => RawObjectType::Hostname,
            "integer" => {
                let bounds = deserialize_integer_bounds(mapping)?;
                RawObjectType::Integer(bounds)
            }
            _ => return Err(Error::custom(format!("unknown object type `{}`", value))),
        };
        Ok(object_type)
    }
}
