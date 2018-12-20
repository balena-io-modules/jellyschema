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
#[derive(Serialize)]
pub struct UiObject<'a>(HashMap<&'a str, UiObjectProperty<'a>>);

impl<'a> UiObject<'a> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Serialize)]
struct UiObjectProperty<'a> {
    #[serde(rename = "ui:help", skip_serializing_if = "Option::is_none")]
    help: Option<&'a str>,
    #[serde(rename = "ui:warning", skip_serializing_if = "Option::is_none")]
    warning: Option<&'a str>,
    #[serde(rename = "ui:description", skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(rename = "ui:widget", skip_serializing_if = "Option::is_none")]
    widget: Option<&'a Widget>,
}

impl<'a> UiObjectProperty<'a> {
    /// Checks if an UI Object is empty
    pub fn is_empty(&self) -> bool {
        self.help.is_none() && self.warning.is_none() && self.description.is_none() && self.widget.is_none()
    }
}
