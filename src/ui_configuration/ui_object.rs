use crate::dsl::validation::ValidatedSchema;
use serde_derive::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct UiObject(HashMap<String, UiObjectProperty>);

#[derive(Serialize)]
struct UiObjectProperty {
    #[serde(rename = "ui:help")]
    help: Option<String>,
    #[serde(rename = "ui:warning")]
    warning: Option<String>,
    #[serde(rename = "ui:description")]
    description: Option<String>,
}

impl From<&ValidatedSchema> for UiObject {
    fn from(schema: &ValidatedSchema) -> Self {
        let mut ui_object_entries = HashMap::new();

        if schema.property_list.clone().is_some() {
            for property_entry in schema.property_list.clone().unwrap().entries {
                ui_object_entries.insert(
                    property_entry.name.to_string(),
                    UiObjectProperty {
                        help: property_entry.property.help,
                        warning: property_entry.property.warning,
                        description: property_entry.property.description,
                    },
                );
            }
        }
        UiObject(ui_object_entries)
    }
}
