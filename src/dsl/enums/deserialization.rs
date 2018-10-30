use crate::dsl::enums::EnumerationValue;
use crate::dsl::enums::EnumerationValues;
use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::RawObjectType;
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
    let mut value = mapping.get(&Value::from("value"));
    let mut title = mapping.get(&Value::from("title"));
    if title.is_none() {
        title = value;
    }
    if value.is_none() {
        value = title;
    }
    if value.is_none() {
        return Err(Error::custom("no value for enum"));
    }
    let value = value.unwrap().as_str().unwrap().to_string();
    let title = title.unwrap().as_str().unwrap().to_string();
    let display_information = DisplayInformation {
        title: Some(title),
        help: None,
        warning: None,
        description: None,
    };
    Ok(EnumerationValue {
        display_information,
        value,
        type_spec: ObjectType::Required(RawObjectType::String),
    })
}
