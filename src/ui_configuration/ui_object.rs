use core::borrow::Borrow;
use crate::dsl::schema::Property;
use crate::dsl::schema::PropertyList;
use crate::dsl::validation::ValidatedSchema;
use serde_derive::Serialize;
use std::collections::HashMap;

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

impl<'a> From<&'a ValidatedSchema> for UiObject<'a> {
    fn from(schema: &'a ValidatedSchema) -> Self {
        match &schema.property_list {
            Some(list) => list.into(),
            None => UiObject(HashMap::new()),
        }
    }
}

impl<'a> From<&'a PropertyList> for UiObject<'a> {
    fn from(list: &'a PropertyList) -> Self {
        let ui_object_entries: Vec<(&str, UiObjectProperty)> = list
            .entries
            .iter()
            .filter_map(|entry| {
                let property: UiObjectProperty = entry.property.borrow().into();
                if !property.is_empty() {
                    Some((entry.name.as_str(), entry.property.borrow().into()))
                } else {
                    None
                }
            })
            .collect();

        let ui_object_entries: HashMap<&str, UiObjectProperty> = ui_object_entries.into_iter().collect();
        UiObject(ui_object_entries)
    }
}

impl<'a> From<&'a Property> for UiObjectProperty<'a> {
    fn from(property: &'a Property) -> Self {
        UiObjectProperty {
            help: property.display_information.help.as_ref().map(|string| string.as_ref()),
            warning: property
                .display_information
                .warning
                .as_ref()
                .map(|string| string.as_str()),
            description: property
                .display_information
                .description
                .as_ref()
                .map(|string| string.as_ref()),
        }
    }
}
