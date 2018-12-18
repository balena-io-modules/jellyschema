use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

use crate::dsl::schema::Annotations;
use crate::dsl::schema::object_types::bounds::EnumerationValue;

pub fn deserialize_enumeration<E>(mapping: &Mapping) -> Result<Option<Vec<EnumerationValue>>, E>
where
    E: Error,
{
    let enumeration_values = deserialize_enumeration_values(&mapping)?;
    let constant_value = deserialize_constant_value(&mapping)?.map(|value| vec![value]);
    if (enumeration_values.is_some()) && constant_value.is_some() {
        return Err(Error::custom("cannot have both enum and const defined"));
    }
    let possible_values = enumeration_values.or(constant_value);
    let possible_values = possible_values.map(|mut list| {
        list.iter_mut()
            .map(|value| {
                if value.annotations.title.is_none() {
                    value.annotations.title = value.value.as_str().map(|s| s.to_string());
                }
                value.clone()
            })
            .collect()
    });
    Ok(possible_values)
}

fn deserialize_enumeration_values<E>(mapping: &Mapping) -> Result<Option<Vec<EnumerationValue>>, E>
where
    E: Error,
{
    let enum_key = Value::from("enum");
    match mapping.get(&enum_key) {
        Some(mapping) => match mapping {
            Value::Sequence(sequence) => {
                let result = sequence
                    .iter()
                    .map(|definition| enumeration_definition_to_enumeration_value(definition))
                    .collect::<Result<Vec<EnumerationValue>, E>>()?;
                if !result.is_empty() {
                    Ok(Some(result))
                } else {
                    Ok(None)
                }
            }
            _ => Err(Error::custom(format!("enum `{:#?}` is not a sequence", mapping))),
        },
        None => Ok(None),
    }
}

fn enumeration_definition_to_enumeration_value<E>(definition: &Value) -> Result<EnumerationValue, E>
where
    E: Error,
{
    match definition {
        Value::String(string) => Ok(string.as_str().into()),
        Value::Mapping(mapping) => Ok(mapping_to_enumeration_value(mapping)?),
        _ => Err(Error::custom(format!("no idea how to deserialize {:#?}", definition))),
    }
}

fn mapping_to_enumeration_value<E>(mapping: &Mapping) -> Result<EnumerationValue, E>
where
    E: Error,
{
    let value = mapping
        .get(&Value::from("value"))
        .ok_or_else(|| Error::custom("when the enumeration is a mapping - expected 'value' to be present"))?;

    let title = mapping.get(&Value::from("title")).map(|value| match value {
        Value::String(string) => Ok(string),
        _ => Err(Error::custom(format!("enum title `{:#?}` must be a string", value))),
    });

    let title = match title {
        None => None,
        Some(result) => match result {
            Ok(value) => Some(value.to_string()),
            Err(e) => return Err(e),
        },
    };

    let annotations = Annotations {
        title,
        help: None,
        warning: None,
        description: None,
    };
    Ok(EnumerationValue {
        annotations,
        value: value.clone(),
    })
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
    let annotations = Annotations {
        title: None,
        help: None,
        warning: None,
        description: None,
    };
    match value {
        None => Ok(None),
        Some(value) => Ok(Some(EnumerationValue { value, annotations })),
    }
}
