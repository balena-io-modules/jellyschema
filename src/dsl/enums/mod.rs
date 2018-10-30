mod from_yaml;

use crate::dsl::object_types::ObjectType;
use crate::dsl::object_types::RawObjectType;
use crate::dsl::schema::DisplayInformation;

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
