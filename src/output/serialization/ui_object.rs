use core::borrow::Borrow;
use std::collections::HashMap;

use crate::dsl::schema::DocumentRoot;
use crate::dsl::schema::Schema;
use crate::dsl::schema::SchemaList;
use crate::output::UiObject;
use crate::output::UiObjectProperty;

impl<'a> From<&'a DocumentRoot> for UiObject<'a> {
    fn from(schema: &'a DocumentRoot) -> Self {
        match &schema.schema {
            Some(schema) => match &schema.children {
                Some(list) => list.into(),
                None => UiObject(HashMap::new()),
            },
            None => UiObject(HashMap::new()),
        }
    }
}

impl<'a> From<&'a SchemaList> for UiObject<'a> {
    fn from(list: &'a SchemaList) -> Self {
        let ui_object_entries: Vec<(&str, UiObjectProperty)> = list
            .entries()
            .iter()
            .filter_map(|entry| {
                let property: UiObjectProperty = entry.schema.borrow().into();
                if !property.is_empty() {
                    Some((entry.name.as_str(), entry.schema.borrow().into()))
                } else {
                    None
                }
            })
            .collect();

        let ui_object_entries: HashMap<&str, UiObjectProperty> = ui_object_entries.into_iter().collect();
        UiObject(ui_object_entries)
    }
}

impl<'a> From<&'a Schema> for UiObjectProperty<'a> {
    fn from(property: &'a Schema) -> Self {
        UiObjectProperty {
            help: property.annotations.help.as_ref().map(|string| string.as_ref()),
            warning: property.annotations.warning.as_ref().map(|string| string.as_str()),
            description: property.annotations.description.as_ref().map(|string| string.as_ref()),
        }
    }
}
