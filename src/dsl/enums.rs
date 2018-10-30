use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::RawObjectType;
use crate::dsl::schema::DisplayInformation;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde_yaml::Mapping;
use serde_yaml::Value;

#[derive(Clone, Debug)]
pub struct EnumerationValues {
    pub possible_values: Vec<EnumerationValue>,
}

#[derive(Clone, Debug)]
pub struct EnumerationValue {
    pub type_spec: ObjectType,
    pub display_information: DisplayInformation,
    pub value: String,
}

impl<'de> Deserialize<'de> for EnumerationValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definitions: Vec<Value> = Vec::deserialize(deserializer)
            .map_err(|e| Error::custom(format!("cannot deserialize sequence - {}", e)))?;
        let enumeration_values: Result<Vec<EnumerationValue>, D::Error> = definitions
            .iter()
            .map(|definition| {
                Ok({
                    if definition.is_string() {
                        let title = definition
                            .as_str()
                            .expect("unwrapping as string failed - serde_yaml inconsistency");
                        title.into()
                    } else if definition.is_mapping() {
                        let mapping = definition
                            .as_mapping()
                            .expect("unwrapping mapping failed - serde_yaml inconsistency");
                        mapping_to_enumeration_value(mapping)?
                    } else {
                        return Err(Error::custom(format!("no idea how to deserialize {:?}", definition)));
                    }
                })
            })
            .collect();

        Ok(EnumerationValues {
            possible_values: enumeration_values?,
        })
    }
}

impl From<&str> for EnumerationValue {
    fn from(value: &str) -> Self {
        let title = value.to_string();
        let display_information = DisplayInformation {
            title: Some(title.clone()),
            help: None,
            warning: None,
            description: None,
        };
        EnumerationValue {
            display_information,
            value: title.clone(),
            type_spec: ObjectType::Required(RawObjectType::String),
        }
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
