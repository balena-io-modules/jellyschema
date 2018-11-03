pub mod deserialization;
mod normalization;
mod validation;

use crate::dsl::schema::DisplayInformation;
use serde_derive::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub enum IntegerBound {
    Inclusive(i64),
    Exclusive(i64),
}

#[derive(Clone, Copy, Debug)]
pub struct IntegerObjectBounds {
    pub minimum: Option<IntegerBound>,
    pub maximum: Option<IntegerBound>,
    pub multiple_of: Option<i64>,
}

impl IntegerBound {
    pub fn inner(&self) -> &i64 {
        match self {
            IntegerBound::Exclusive(value) => value,
            IntegerBound::Inclusive(value) => value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumerationValues {
    pub possible_values: Vec<EnumerationValue>,
}

#[derive(Clone, Debug)]
pub struct EnumerationValue {
    pub display_information: DisplayInformation,
    // TODO: value should nor really be optional, semantically speaking
    pub value: Option<String>,
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
            value: Some(title.clone()),
        }
    }
}
