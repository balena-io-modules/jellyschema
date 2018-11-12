pub mod deserialization;

use crate::dsl::schema::DisplayInformation;
use crate::dsl::schema::Property;
use regex::Regex;
use serde_derive::Serialize;

// TODO: impl serialize separately, to not have serialization code in the `dsl` module
#[derive(Clone, Debug, Serialize)]
pub enum IntegerBound {
    Inclusive(i64),
    Exclusive(i64),
}

#[derive(Clone, Debug)]
pub struct IntegerObjectBounds {
    pub minimum: Option<IntegerBound>,
    pub maximum: Option<IntegerBound>,
    pub multiple_of: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct ArrayObjectBounds {
    pub minimum_number_of_items: Option<i64>,
    pub maximum_number_of_items: Option<i64>,
    pub items: Option<ArrayItemObjectBounds>,
    pub unique_items: Option<ArrayUniqueItemBound>,
    pub additional_items: Option<Property>,
}

#[derive(Clone, Debug)]
pub enum ArrayUniqueItemBound {
    All,
    Specific(Vec<String>),
}

#[derive(Clone, Debug)]
pub enum ArrayItemObjectBounds {
    AllItems(Property),
    RespectiveItems(Vec<Property>),
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
pub struct StringLength {
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
}

#[derive(Clone, Debug)]
pub enum StringObjectBounds {
    PossibleValues(Vec<EnumerationValue>),
    Pattern(Regex),
    Length(StringLength),
}

#[derive(Clone, Debug)]
pub enum BooleanObjectBounds {
    DefaultValue(bool),
}

#[derive(Clone, Debug)]
pub struct EnumerationValue {
    pub display_information: DisplayInformation,
    pub value: String,
}

impl From<&str> for EnumerationValue {
    fn from(value: &str) -> Self {
        let value = value.to_string();
        let display_information = DisplayInformation {
            title: Some(value.clone()),
            help: None,
            warning: None,
            description: None,
        };
        EnumerationValue {
            value: value.clone(),
            display_information,
        }
    }
}
