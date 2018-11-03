use crate::dsl::object_types::bounds::EnumerationValue;
use crate::dsl::object_types::bounds::IntegerBound;
use crate::dsl::object_types::bounds::IntegerObjectBounds;
use crate::dsl::object_types::bounds::StringObjectBounds;
use crate::dsl::object_types::deserialization::deserialize_integer;
use crate::dsl::schema::DisplayInformation;
use heck::MixedCase;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde_yaml::Mapping;
use serde_yaml::Value;

impl<'de> Deserialize<'de> for StringObjectBounds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definitions: Vec<Value> = Vec::deserialize(deserializer)
            .map_err(|e| Error::custom(format!("cannot deserialize sequence - {}", e)))?;
        let enumeration_values: Result<Vec<EnumerationValue>, D::Error> = definitions
            .iter()
            .map(|definition| Ok(enumeration_definition_to_enumeration_value(definition)?))
            .collect();

        Ok(StringObjectBounds {
            possible_values: enumeration_values?,
        })
    }
}

pub fn deserialize_enumeration_values<E>(mapping: &Mapping) -> Result<Option<StringObjectBounds>, E>
where
    E: Error,
{
    let enum_key = Value::from("enum");
    let enums = mapping.get(&enum_key).map_or(Ok(None), |value| {
        serde_yaml::from_value(value.clone()).map_err(|e| {
            Error::custom(format!(
                "cannot deserialize list of enumeration values: {:#?} - {}",
                value, e
            ))
        })
    })?;

    let constant_key = Value::from("const");
    let constant = mapping.get(&constant_key).map_or(Ok(None), |value| {
        serde_yaml::from_value(value.clone())
            .map_err(|e| Error::custom(format!("cannot deserialize constant specifier: {:?} - {}", value, e)))
    })?;

    if enums.is_some() && constant.is_some() {
        return Err(Error::custom("cannot have both enum and const defined"));
    }

    if constant.is_some() {
        return Ok(Some({
            let display_information = DisplayInformation {
                title: None,
                help: None,
                warning: None,
                description: None,
            };

            StringObjectBounds {
                possible_values: vec![EnumerationValue {
                    value: constant.clone(),
                    display_information,
                }],
            }
        }));
    }

    Ok(enums)
}

pub fn deserialize_integer_bounds<E>(mapping: &Mapping) -> Result<Option<IntegerObjectBounds>, E>
where
    E: Error,
{
    let maximum = deserialize_integer_bound("maximum", mapping)?;
    let minimum = deserialize_integer_bound("minimum", mapping)?;
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
    let value = value.map(|value| {
        value
            .as_str()
            .expect("serde_yaml type inconsistence on value")
            .to_string()
    });
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
