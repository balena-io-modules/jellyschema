use crate::dsl::schema::PropertyList;
use serde_derive::Serialize;
use std::collections::HashMap;
pub mod generator;
mod serialization;

pub struct JsonSchema<'a> {
    version: u64,
    schema_url: &'a str,
    // FIXME: use RawObjectType and reuse type_spec, title and properties from Property
    type_spec: String,
    title: &'a str,
    properties: Option<&'a PropertyList>,
}

#[derive(Serialize)]
pub struct UiObject<'a>(HashMap<&'a str, UiObjectProperty<'a>>);

impl<'a> UiObject<'a> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Serialize)]
struct UiObjectProperty<'a> {
    #[serde(rename = "ui:help")]
    help: Option<&'a str>,
    #[serde(rename = "ui:warning")]
    warning: Option<&'a str>,
    #[serde(rename = "ui:description")]
    description: Option<&'a str>,
}

impl<'a> UiObjectProperty<'a> {
    pub fn is_empty(&self) -> bool {
        self.help.is_none() && self.warning.is_none() && self.description.is_none()
    }
}
