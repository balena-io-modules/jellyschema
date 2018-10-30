use crate::dsl::schema::DisplayInformation;
use crate::dsl::types::EnumerationValue;
use crate::dsl::types::ObjectType;
use crate::dsl::types::TypeSpec;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde_yaml::Value;

#[derive(Clone, Debug)]
pub struct EnumerationValues {
    pub possible_values: Vec<EnumerationValue>,
}

impl<'de> Deserialize<'de> for EnumerationValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let definitions: Vec<Value> = Vec::deserialize(deserializer)
            .map_err(|e| Error::custom(format!("cannot deserialize sequence - {}", e)))?;
        let mut enumeration_values = vec![];
        for definition in definitions {
            let enumeration_value = {
                if definition.is_string() {
                    let title = definition
                        .as_str()
                        .expect("unwrapping as string failed - serde_yaml inconsistency")
                        .to_string();
                    let display_information = DisplayInformation {
                        title: Some(title.clone()),
                        help: None,
                        warning: None,
                        description: None,
                    };
                    EnumerationValue {
                        display_information,
                        value: title.clone(),
                        type_spec: TypeSpec::Required(ObjectType::String),
                    }
                } else if definition.is_mapping() {
                    // value or title
                    let mapping = definition
                        .as_mapping()
                        .expect("unwrapping mapping failed - serde_yaml inconsistency");
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
                    EnumerationValue {
                        display_information,
                        value,
                        type_spec: TypeSpec::Required(ObjectType::String),
                    }
                } else {
                    return Err(Error::custom(format!("no idea how to deserialize {:?}", definition)));
                }
            };
            enumeration_values.push(enumeration_value);
        }
        Ok(EnumerationValues {
            possible_values: enumeration_values,
        })
    }
}
