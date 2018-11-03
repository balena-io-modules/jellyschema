use crate::dsl::object_types::bounds::EnumerationValue;
use crate::dsl::object_types::bounds::EnumerationValues;
use crate::dsl::schema::DisplayInformation;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde_yaml::Mapping;
use serde_yaml::Value;

impl<'de> Deserialize<'de> for EnumerationValues {
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

        Ok(EnumerationValues {
            possible_values: enumeration_values?,
        })
    }
}

pub fn deserialize_enumeration_values<E>(mapping: &Mapping) -> Result<Option<EnumerationValues>, E>
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

            EnumerationValues {
                possible_values: vec![EnumerationValue {
                    value: constant.clone(),
                    display_information,
                }],
            }
        }));
    }

    Ok(enums)
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
