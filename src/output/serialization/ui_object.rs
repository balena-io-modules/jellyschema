use std::collections::HashMap;
use serde::ser::SerializeMap;

use crate::dsl::schema::DocumentRoot;
use crate::dsl::schema::Schema;
use crate::dsl::schema::SchemaList;
use crate::output::UiObject;
use crate::output::UiObjectProperty;
use crate::output::UiObjectRoot;
use serde::Serialize;
use serde::Serializer;
use crate::dsl::schema::KeysSchema;
use crate::output::UiOptions;

impl From<DocumentRoot> for UiObjectRoot {
    fn from(schema: DocumentRoot) -> UiObjectRoot {
        UiObjectRoot(schema.schema.map(|schema| schema.into()))
    }
}

impl From<SchemaList> for UiObject {
    fn from(list: SchemaList) -> Self {
        let ui_object_entries: Vec<(String, UiObjectProperty)> = list
            .entries()
            .iter()
            .filter_map(|entry| {
                let property: UiObjectProperty = entry.schema.clone().into();
                if !property.is_empty() {
                    Some((entry.name.clone(), entry.schema.clone().into()))
                } else {
                    None
                }
            })
            .collect();

        let ui_object_entries: HashMap<String, UiObjectProperty> = ui_object_entries.into_iter().collect();
        UiObject(ui_object_entries)
    }
}

impl From<Schema> for UiObjectProperty {
    fn from(schema: Schema) -> Self {
        let annotations = schema.annotations.clone();
        let help = annotations.help;
        let warning = annotations.warning;
        let description = annotations.description;
        let widget = annotations.widget;
        let keys_values = schema.dynamic.map(|keys_values| keys_values.keys);

        let children = schema.children.map(|children| children.into());
        let default_ui_options = UiOptions::default();

        let ui_options = UiOptions {
            removable: annotations.removable.unwrap_or(default_ui_options.removable),
            addable: annotations.addable.unwrap_or(default_ui_options.addable),
            orderable: annotations.orderable.unwrap_or(default_ui_options.orderable),
        };

        let ui_options = if ui_options == default_ui_options {
            None
        } else {
            Some(ui_options)
        };

        UiObjectProperty {
            help,
            warning,
            description,
            widget,
            properties: children,
            keys: keys_values,
            ui_options,
        }
    }
}

impl Serialize for KeysSchema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        if let Some(title) = &self.title {
            let mut keys_definition = HashMap::new();
            keys_definition.insert("ui:title", title);
            map.serialize_entry("ui:keys", &keys_definition)?;
        }
        map.end()
    }
}
