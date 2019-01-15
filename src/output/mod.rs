//! A module containing output generator for the JSON Schema & UI Object
use std::collections::HashMap;

use serde_derive::Serialize;

use crate::dsl::schema::Schema;
use crate::dsl::schema::when::DependencyGraph;
use crate::dsl::schema::Widget;

pub mod generator;
mod serialization;

/// JSON Schema wrapper
pub struct JsonSchema<'a> {
    version: u64,
    schema_url: &'a str,
    root: Option<&'a Schema>,
    dependencies: Option<&'a DependencyGraph>,
}

/// UI Object wrapper
#[derive(Clone, Serialize)]
pub struct UiObject(HashMap<String, UiObjectProperty>);

impl UiObject {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Serialize)]
struct UiObjectProperty {
    #[serde(rename = "ui:help", skip_serializing_if = "Option::is_none")]
    help: Option<String>,
    #[serde(rename = "ui:warning", skip_serializing_if = "Option::is_none")]
    warning: Option<String>,
    #[serde(rename = "ui:description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "ui:widget", skip_serializing_if = "Option::is_none")]
    widget: Option<Widget>,
    #[serde(flatten)]
    children: Option<UiObject>
}

impl UiObjectProperty {
    /// Checks if an UI Object is empty
    pub fn is_empty(&self) -> bool {
        self.help.is_none() && self.warning.is_none() && self.description.is_none() && self.widget.is_none()
    }
}
