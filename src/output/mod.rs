//! A module containing output generator for the JSON Schema & UI Object
use std::collections::HashMap;

use serde_derive::Serialize;

use crate::dsl::schema::Schema;
use crate::dsl::schema::when::DependencyGraph;

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
    #[serde(rename = "ui:help")]
    help: Option<&'a str>,
    #[serde(rename = "ui:warning")]
    warning: Option<&'a str>,
    #[serde(rename = "ui:description")]
    description: Option<&'a str>,
}

impl<'a> UiObjectProperty<'a> {
    /// Checks if an UI Object is empty
    pub fn is_empty(&self) -> bool {
        self.help.is_none() && self.warning.is_none() && self.description.is_none()
    }
}
