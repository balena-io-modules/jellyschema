use core::borrow::Borrow;
use std::collections::HashMap;

use crate::dsl::schema::Property;
use crate::dsl::schema::PropertyList;
use crate::dsl::schema::SourceSchema;
use crate::ui_configuration::UiObject;
use crate::ui_configuration::UiObjectProperty;

impl<'a> From<&'a SourceSchema> for UiObject<'a> {
    fn from(schema: &'a SourceSchema) -> Self {
        match &schema.self_property {
            Some(property) => match &property.property_list {
                Some(list) => list.into(),
                None => UiObject(HashMap::new()),
            },
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
