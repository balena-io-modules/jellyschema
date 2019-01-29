//! A `bound` is anything that restricts the instance value, please see `validation keywords` in the spec
use regex::Regex;
use serde_derive::Serialize;

use crate::dsl::schema::Annotations;
use crate::dsl::schema::Schema;

pub mod deserialization;
mod enums;

#[derive(Clone, Debug)]
pub struct BooleanObjectBounds(pub Vec<EnumerationValue>);

#[derive(Clone, Debug)]
pub enum StringObjectBounds {
    Enumeration(Vec<EnumerationValue>),
    Value(StringValueObjectBounds),
}

#[derive(Clone, Debug)]
pub struct StringValueObjectBounds {
    pub pattern: Option<Regex>,
    pub length: Option<StringLength>,
}

#[derive(Clone, Debug)]
pub struct IntegerValueConditionObjectBounds {
    pub minimum: Option<IntegerBound>,
    pub maximum: Option<IntegerBound>,
    pub multiple_of: Option<i64>,
}

#[derive(Clone, Debug)]
pub enum IntegerObjectBounds {
    Conditions(IntegerValueConditionObjectBounds),
    Enumeration(Vec<EnumerationValue>),
}

#[derive(Clone, Debug)]
pub struct ArrayObjectBounds {
    pub minimum_number_of_items: Option<i64>,
    pub maximum_number_of_items: Option<i64>,
    pub items: Option<ArrayItemObjectBounds>,
    pub unique_items: Option<ArrayUniqueItemBound>,
}

#[derive(Clone, Debug)]
pub enum ArrayUniqueItemBound {
    All,
    Specific(Vec<String>),
}

#[derive(Clone, Debug)]
pub enum ArrayItemObjectBounds {
    AllItems(Box<Schema>),
    AnyItems(Vec<Schema>),
}

// TODO: impl serialize separately, to not have serialization code in the `dsl` module
#[derive(Clone, Debug, Serialize)]
pub enum IntegerBound {
    Inclusive(i64),
    Exclusive(i64),
}

impl IntegerBound {
    pub fn inner(&self) -> &i64 {
        match self {
            IntegerBound::Exclusive(value) => value,
            IntegerBound::Inclusive(value) => value,
        }
    }
}

impl IntegerObjectBounds {
    pub fn with_defaults(self, defaults: IntegerObjectBounds) -> IntegerObjectBounds {
        if let IntegerObjectBounds::Conditions(old) = self.clone() {
            if let IntegerObjectBounds::Conditions(default) = defaults {
                let new_conditions = IntegerValueConditionObjectBounds {
                    minimum: old.clone().minimum.or(default.minimum),
                    maximum: old.clone().maximum.or(default.maximum),
                    multiple_of: old.clone().multiple_of.or(default.multiple_of),
                };
                IntegerObjectBounds::Conditions(new_conditions)
            } else {
                self
            }
        } else {
            self
        }
    }
}

#[derive(Clone, Debug)]
pub struct StringLength {
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct DefaultValue(serde_yaml::Value);

impl DefaultValue {
    pub fn value(&self) -> &serde_yaml::Value {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct EnumerationValue {
    pub annotations: Annotations,
    pub value: serde_yaml::Value,
}

impl From<&str> for EnumerationValue {
    fn from(value: &str) -> Self {
        let value = value.to_string();
        let annotations = Annotations::with_title_option(Some(value.clone()));
        EnumerationValue {
            value: value.into(),
            annotations,
        }
    }
}
