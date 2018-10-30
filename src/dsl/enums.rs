use crate::dsl::types::EnumerationValue;
use crate::dsl::types::ObjectType;
use serde::de::Error;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde_yaml::Sequence;
use serde_yaml::Value;
use std::fmt;
use std::fmt::Formatter;
use crate::dsl::schema::DisplayInformation;
use crate::dsl::types::TypeSpec;

#[derive(Clone, Debug)]
pub struct EnumerationValues {
    pub possible_values: Vec<EnumerationValue>,
}

impl<'de> Deserialize<'de> for EnumerationValues {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values: Vec<Value> =
            Vec::deserialize(deserializer).map_err(|e| Error::custom("cannot deserialize sequence"))?;
        let mut object_values = vec![];
        for value in values {
            let value = {
                if value.is_string() {
                    let display_information = DisplayInformation {
                        title: Some(value.as_str().unwrap().to_string()),
                        help: None,
                        warning: None,
                        description: None
                    };
                    EnumerationValue {
                        display_information,
                        type_spec: TypeSpec::Required(ObjectType::String)
                    }
                } else if value.is_mapping() {
                    let display_information = DisplayInformation {
                        title: Some(value.as_mapping().unwrap().len().to_string()),
                        help: None,
                        warning: None,
                        description: None
                    };
                    EnumerationValue {
                        display_information,
                        type_spec: TypeSpec::Required(ObjectType::String)
                    }
                } else {
                    return Err(Error::custom(format!("no idea how to deserialize {:?}", value)));
                }
            };
            object_values.push(value);
        }
        Ok(EnumerationValues {
            possible_values: object_values,
        })
    }
}
