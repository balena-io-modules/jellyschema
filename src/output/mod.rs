//! A module containing output generator for the JSON Schema & UI Object
use std::collections::HashMap;

use serde_derive::Serialize;

use crate::dsl::schema::Schema;
use crate::dsl::schema::when::DependencyGraph;
use crate::dsl::schema::Widget;
use crate::dsl::schema::KeysSchema;

pub mod generator;
mod serialization;

/// JSON Schema wrapper
pub struct JsonSchema<'a> {
    version: u64,
    schema_url: &'a str,
    root: Option<&'a Schema>,
    dependencies: Option<&'a DependencyGraph>,
}

/// It's different than UiObject as the root is nameless in the output
#[derive(Clone, Debug, Serialize)]
pub struct UiObjectRoot(Option<UiObjectProperty>);

/// UI Object wrapper
#[derive(Clone, Debug, Serialize)]
pub struct UiObject(HashMap<String, UiObjectProperty>);

impl UiObject {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct UiOptions {
    removable: bool,
    addable: bool,
    orderable: bool,
}

#[derive(Clone, Debug, Serialize)]
struct UiObjectProperty {
    #[serde(rename = "ui:help", skip_serializing_if = "Option::is_none")]
    help: Option<String>,
    #[serde(rename = "ui:warning", skip_serializing_if = "Option::is_none")]
    warning: Option<String>,
    #[serde(rename = "ui:description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "ui:placeholder", skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
    #[serde(rename = "ui:widget", skip_serializing_if = "Option::is_none")]
    widget: Option<Widget>,
    #[serde(flatten)]
    properties: Option<UiObject>,
    #[serde(flatten)]
    keys: Option<KeysSchema>,

    #[serde(rename = "ui:options", skip_serializing_if = "Option::is_none")]
    ui_options: Option<UiOptions>,
}

impl UiObjectProperty {
    /// Checks if an UI Object is empty
    pub fn is_empty(&self) -> bool {
        self.help.is_none()
            && self.warning.is_none()
            && self.description.is_none()
            && self.widget.is_none()
            && self.properties.is_none()
            && self.keys.is_none()
            && self.ui_options.is_none()
            && self.placeholder.is_none()
    }
}

impl UiObjectRoot {
    pub fn is_empty(&self) -> bool {
        match &self.0 {
            None => true,
            Some(property) => property.is_empty(),
        }
    }
}

impl Default for UiOptions {
    fn default() -> Self {
        UiOptions {
            removable: true,
            addable: true,
            orderable: true,
        }
    }
}
