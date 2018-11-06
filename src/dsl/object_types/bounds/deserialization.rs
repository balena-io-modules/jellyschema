use crate::dsl::object_types::bounds::EnumerationValue;
use crate::dsl::object_types::bounds::IntegerBound;
use crate::dsl::object_types::bounds::IntegerObjectBounds;
use crate::dsl::object_types::bounds::StringLength;
use crate::dsl::object_types::bounds::StringObjectBounds;
use crate::dsl::object_types::deserialization::deserialize_integer;
use crate::dsl::schema::DisplayInformation;
use heck::MixedCase;
use regex::Regex;
use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

// fixme this method is not the best method ;)
pub fn deserialize_string_object_bounds<E>(mapping: &Mapping) -> Result<Option<StringObjectBounds>, E>
where
    E: Error,
{
    // fixme: there seems to be some exclusivity going on between the flags - fix with types ?
    let enumeration_values = deserialize_enumeration_values(&mapping)?;
    let constant_value = deserialize_constant_value(&mapping)?;
    let pattern = deserialize_pattern(&mapping)?;

    if (!enumeration_values.is_empty()) && constant_value.is_some() {
        return Err(Error::custom("cannot have both enum and const defined"));
    }

    if pattern.is_some() && (constant_value.is_some() || !enumeration_values.is_empty()) {
        return Err(Error::custom("cannot have both pattern set and enum/const bound"));
    }

    if pattern.is_some() {
        return Ok(Some(StringObjectBounds::Pattern(pattern.unwrap())));
    }

    let max_length = deserialize_integer("maxLength", &mapping)?;
    let min_length = deserialize_integer("minLength", &mapping)?;
    if max_length.is_some() || min_length.is_some() {
        if constant_value.is_some() {
            return Err(Error::custom("cannot have both length set and const bound"));
        }
        if !enumeration_values.is_empty() {
            return Err(Error::custom("cannot have both length set and enum bound"));
        }
        return Ok(Some(StringObjectBounds::Length(StringLength {
            minimum: min_length,
            maximum: max_length,
        })));
    }

    if constant_value.is_some() {
        Ok(Some(StringObjectBounds::PossibleValues(vec![constant_value.unwrap()])))
    } else {
        Ok(Some(StringObjectBounds::PossibleValues(enumeration_values)))
    }
}

pub fn deserialize_integer_bounds<E>(mapping: &Mapping) -> Result<Option<IntegerObjectBounds>, E>
where
    E: Error,
{
    let maximum = deserialize_integer_bound("maximum", mapping)?;
    let minimum = deserialize_integer_bound("minimum", mapping)?;
    let multiple_of = deserialize_integer("multipleOf", mapping)?;

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

fn deserialize_enumeration_values<E>(mapping: &Mapping) -> Result<Vec<EnumerationValue>, E>
where
    E: Error,
{
    let enum_key = Value::from("enum");
    let enums = mapping.get(&enum_key);
    if enums.is_none() {
        return Ok(vec![]);
    }

    let definitions = enums.unwrap().as_sequence();

    if definitions.is_none() {
        return Err(Error::custom(format!(
            "cannot deserialize sequence - {:#?} is not a sequence",
            enums
        )));
    }

    definitions
        .unwrap()
        .iter()
        .map(|definition| Ok(enumeration_definition_to_enumeration_value(definition)?))
        .collect()
}

fn deserialize_pattern<E>(mapping: &Mapping) -> Result<Option<Regex>, E>
where
    E: Error,
{
    let pattern_key = Value::from("pattern");
    let pattern = mapping.get(&pattern_key);
    if pattern.is_none() {
        return Ok(None);
    }

    let pattern_text = pattern.unwrap().as_str();
    if pattern_text.is_none() {
        return Err(Error::custom(format!("pattern `{:#?}` must be a string", pattern)));
    }
    Ok(Some(Regex::new(pattern_text.unwrap()).map_err(|e| {
        Error::custom(format!("pattern `{:?}` is not a regex - {}", pattern_text, e))
    })?))
}

fn deserialize_constant_value<E>(mapping: &Mapping) -> Result<Option<EnumerationValue>, E>
where
    E: Error,
{
    let constant_key = Value::from("const");
    let value = mapping.get(&constant_key).map_or(Ok(None), |value| {
        serde_yaml::from_value(value.clone())
            .map_err(|e| Error::custom(format!("cannot deserialize constant specifier: {:?} - {}", value, e)))
    })?;
    let display_information = DisplayInformation {
        title: None,
        help: None,
        warning: None,
        description: None,
    };
    match value {
        None => Ok(None),
        Some(value) => Ok(Some(EnumerationValue {
            value,
            display_information,
        })),
    }
}

fn enumeration_definition_to_enumeration_value<E>(definition: &Value) -> Result<EnumerationValue, E>
where
    E: Error,
{
    // FIXME: readability
    if definition.is_string() {
        Ok(definition
            .as_str()
            .expect("unwrapping as string failed - serde_yaml inconsistency")
            .into())
    } else if definition.is_mapping() {
        let mapping = definition
            .as_mapping()
            .expect("unwrapping mapping failed - serde_yaml inconsistency");
        Ok(mapping_to_enumeration_value(mapping)?)
    } else {
        Err(Error::custom(format!("no idea how to deserialize {:?}", definition)))
    }
}

fn mapping_to_enumeration_value<E>(mapping: &Mapping) -> Result<EnumerationValue, E>
where
    E: Error,
{
    let value = mapping.get(&Value::from("value"));
    let title = mapping.get(&Value::from("title"));
    let value = value
        .map(|value| {
            value
                .as_str()
                .expect("serde_yaml type inconsistence on value")
                .to_string()
        })
        .ok_or_else(|| Error::custom("when the enumeration is a mapping - expected 'value' to be present"))?;
    let title = title.map(|value| {
        value
            .as_str()
            .expect("serde_yaml type inconsistence on title")
            .to_string()
    });
    let display_information = DisplayInformation {
        title,
        help: None,
        warning: None,
        description: None,
    };
    Ok(EnumerationValue {
        display_information,
        value,
    })
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
    if normal.is_some() {
        return Ok(Some(IntegerBound::Inclusive(normal.unwrap())));
    }
    if exclusive.is_some() {
        return Ok(Some(IntegerBound::Exclusive(exclusive.unwrap())));
    }
    Ok(None)
}
