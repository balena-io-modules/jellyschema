use heck::MixedCase;
use regex::Regex;
use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

use crate::dsl::schema::deserialization::deserialize_schema;
use crate::dsl::schema::object_types::bounds::ArrayItemObjectBounds;
use crate::dsl::schema::object_types::bounds::ArrayObjectBounds;
use crate::dsl::schema::object_types::bounds::ArrayUniqueItemBound;
use crate::dsl::schema::object_types::bounds::BooleanObjectBounds;
use crate::dsl::schema::object_types::bounds::enums::deserialize_enumeration;
use crate::dsl::schema::object_types::bounds::IntegerBound;
use crate::dsl::schema::object_types::bounds::IntegerObjectBounds;
use crate::dsl::schema::object_types::bounds::StringLength;
use crate::dsl::schema::object_types::bounds::StringObjectBounds;
use crate::dsl::schema::object_types::deserialization::deserialize_integer;
use crate::dsl::schema::Schema;
use crate::dsl::schema::object_types::bounds::IntegerValueConditionObjectBounds;

pub fn deserialize_string_object_bounds<E>(mapping: &Mapping) -> Result<Option<StringObjectBounds>, E>
where
    E: Error,
{
    let possible_values = deserialize_enumeration(&mapping)?;

    let pattern = deserialize_pattern(&mapping)?;
    let length = deserialize_length_bounds(&mapping)?;
    if possible_values.is_some() && pattern.is_some() {
        return Err(Error::custom("cannot have both pattern set and enum/const bound"));
    }
    if possible_values.is_some() && length.is_some() {
        return Err(Error::custom("cannot have both length set and enum/const bound"));
    }
    let result = {
        if let Some(values) = possible_values {
            Some(StringObjectBounds::List(values))
        } else if let Some(pattern) = pattern {
            Some(StringObjectBounds::Pattern(pattern))
        } else if let Some(length) = length {
            Some(length)
        } else {
            None
        }
    };

    Ok(result)
}

pub fn deserialize_length_bounds<E>(mapping: &Mapping) -> Result<Option<StringObjectBounds>, E>
where
    E: Error,
{
    let max_length = deserialize_integer("maxLength", &mapping)?;
    let min_length = deserialize_integer("minLength", &mapping)?;
    if max_length.is_some() || min_length.is_some() {
        Ok(Some(StringObjectBounds::Length(StringLength {
            minimum: min_length,
            maximum: max_length,
        })))
    } else {
        Ok(None)
    }
}

pub fn deserialize_integer_bounds_with_defaults<E>(
    defaults: IntegerObjectBounds,
    mapping: &Mapping,
) -> Result<IntegerObjectBounds, E>
where
    E: Error,
{
    let bounds = deserialize_integer_bounds(mapping)?;

    match bounds {
        None => Ok(defaults),
        Some(bounds) => Ok(bounds.with_defaults(defaults)),
    }
}

pub fn deserialize_integer_bounds<E>(mapping: &Mapping) -> Result<Option<IntegerObjectBounds>, E>
where
    E: Error,
{
    let maximum = deserialize_integer_bound("max", mapping)?;
    let minimum = deserialize_integer_bound("min", mapping)?;
    let multiple_of = deserialize_integer("multipleOf", mapping)?;
    let possible_values = deserialize_enumeration(&mapping)?;
    let integer_bound_present = maximum.is_some() || minimum.is_some() || multiple_of.is_some();

    if integer_bound_present && possible_values.is_some() {
        return Err(Error::custom(
            "cannot have both min/max/multiple bounds set and enum/const bound set at the same time",
        ));
    }

    if integer_bound_present {
        return Ok(Some(IntegerObjectBounds::Conditions(
            IntegerValueConditionObjectBounds {
                minimum,
                maximum,
                multiple_of,
            },
        )));
    }

    if let Some(values) = possible_values {
        return Ok(Some(IntegerObjectBounds::List(values)));
    }

    Ok(None)
}

pub fn deserialize_boolean_object_bounds<E>(mapping: &Mapping) -> Result<Option<BooleanObjectBounds>, E>
where
    E: Error,
{
    let default_key = Value::from("default");

    match mapping.get(&default_key) {
        Some(default) => match default {
            Value::Bool(value) => Ok(Some(BooleanObjectBounds::DefaultValue(*value))),
            _ => Err(Error::custom(format!(
                "cannot deserialize default value - {:#?} is not a boolean",
                default
            ))),
        },
        None => Ok(None),
    }
}

pub fn deserialize_array_object_bounds<E>(mapping: &Mapping) -> Result<Option<ArrayObjectBounds>, E>
where
    E: Error,
{
    let maximum_number_of_items = deserialize_integer("maxItems", mapping)?;
    let minimum_number_of_items = deserialize_integer("minItems", mapping)?;
    let items = deserialize_array_item_bounds(mapping)?;
    let unique_items = deserialize_array_unique_items_bounds(mapping)?;

    if maximum_number_of_items.is_some() || minimum_number_of_items.is_some() || items.is_some() {
        return Ok(Some(ArrayObjectBounds {
            minimum_number_of_items,
            maximum_number_of_items,
            items,
            unique_items,
        }));
    }
    Ok(None)
}

fn deserialize_array_unique_items_bounds<E>(mapping: &Mapping) -> Result<Option<ArrayUniqueItemBound>, E>
where
    E: Error,
{
    match mapping.get(&Value::from("uniqueItems")) {
        None => Ok(None),
        Some(items) => match items {
            Value::Bool(value) => {
                if *value {
                    Ok(Some(ArrayUniqueItemBound::All))
                } else {
                    Ok(None)
                }
            }
            Value::Sequence(sequence) => {
                let names = sequence
                    .iter()
                    .map(|item| match item {
                        Value::String(name) => Ok(name.to_string()),
                        _ => Err(Error::custom(
                            "`uniqueItems` entry cannot be anything else than a string",
                        )),
                    })
                    .collect::<Result<Vec<String>, E>>();
                Ok(Some(ArrayUniqueItemBound::Specific(names?)))
            }
            _ => Err(Error::custom(format!(
                "unsupported shape of the `uniqueItems` {:#?}",
                items
            ))),
        },
    }
}

fn deserialize_array_item_bounds<E>(mapping: &Mapping) -> Result<Option<ArrayItemObjectBounds>, E>
where
    E: Error,
{
    match mapping.get(&Value::from("items")) {
        None => Ok(None),
        Some(properties) => match properties {
            Value::Mapping(_) => Ok(Some(ArrayItemObjectBounds::AllItems(Box::new(deserialize_schema(
                properties,
            )?)))),
            Value::Sequence(sequence) => Ok(Some(ArrayItemObjectBounds::AnyItems(
                sequence
                    .iter()
                    .map(|entry| deserialize_schema(entry))
                    .collect::<Result<Vec<Schema>, E>>()?,
            ))),
            _ => Err(Error::custom("`items` must be either a schema or array of schemas")),
        },
    }
}

fn deserialize_pattern<E>(mapping: &Mapping) -> Result<Option<Regex>, E>
where
    E: Error,
{
    let pattern_key = Value::from("pattern");
    match mapping.get(&pattern_key) {
        Some(pattern) => match pattern {
            Value::String(string) => {
                Ok(Some(Regex::new(string).map_err(|e| {
                    Error::custom(format!("pattern `{:?}` is not a regex - {}", pattern, e))
                })?))
            }
            _ => Err(Error::custom(format!("pattern `{:#?}` must be a string", pattern))),
        },
        None => Ok(None),
    }
}

fn deserialize_integer_bound<E>(name: &str, mapping: &Mapping) -> Result<Option<IntegerBound>, E>
where
    E: Error,
{
    let normal = deserialize_integer(name, mapping)?;
    let exclusive = deserialize_integer(&("exclusive ".to_string() + name).to_mixed_case(), mapping)?;
    if normal.is_some() && exclusive.is_some() {
        return Err(Error::custom("cannot have both {} and exclusive {} set"));
    }
    if let Some(normal) = normal {
        return Ok(Some(IntegerBound::Inclusive(normal)));
    }
    if let Some(exclusive) = exclusive {
        return Ok(Some(IntegerBound::Exclusive(exclusive)));
    }
    Ok(None)
}
