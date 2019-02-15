use std::string::ToString;

use serde::ser::{Error, SerializeMap};
use serde::{Serialize, Serializer};
use serde_json::{json, Map, Value};

use crate::schema::{PrimitiveType, Schema};

pub struct UiSchema<'a> {
    schema: &'a Schema,
}

impl<'a> Serialize for UiSchema<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        serialize_as_ui_schema(&self.schema, &mut map)?;
        map.end()
    }
}

impl<'a> UiSchema<'a> {
    pub fn new(schema: &'a Schema) -> Self {
        UiSchema { schema }
    }
}

fn serialize_widget(schema: &Schema, map: &mut Map<String, Value>) {
    // TODO Is there any other way how to express this? To avoid clashes
    //      of `type: password` & `hidden: true` for example
    if schema.write_only() {
        map.insert("ui:widget".to_string(), json!("password"));
    }

    if schema.hidden() {
        map.insert("ui:widget".to_string(), json!("hidden"));
    }

    // Do this as a last thing, because `type` is preferred and if it clashes,
    // we'd like to have a widget based on the `type`
    let primitive_type = schema.r#type().primitive_type();
    match primitive_type {
        PrimitiveType::Password => map.insert("ui:widget".to_string(), json!("password")),
        PrimitiveType::Text => map.insert("ui:widget".to_string(), json!("textarea")),
        _ => None,
    };
}

fn serialize_ui_options(schema: &Schema, map: &mut Map<String, Value>) {
    let addable = schema.addable().unwrap_or(true);
    let removable = schema.removable().unwrap_or(true);
    let orderable = schema.orderable().unwrap_or(true);

    if !(addable && removable && orderable) {
        map.insert(
            "ui:options".to_string(),
            json!({
                "addable": addable,
                "orderable": orderable,
                "removable": removable
            }),
        );
    }
}

fn serialize_annotations(schema: &Schema, map: &mut Map<String, Value>) {
    if let Some(help) = schema.help() {
        map.insert("ui:help".to_string(), Value::String(help.to_string()));
    }

    if let Some(warning) = schema.warning() {
        map.insert("ui:warning".to_string(), Value::String(warning.to_string()));
    }

    if let Some(placeholder) = schema.placeholder() {
        map.insert("ui:placeholder".to_string(), Value::String(placeholder.to_string()));
    }
}

fn serialize_properties(schema: &Schema, map: &mut Map<String, Value>) {
    let mut order = vec![];
    let mut properties = Map::<String, Value>::new();

    for property in schema.properties().iter() {
        order.push(property.name());

        let mut property_map = Map::<String, Value>::new();
        serialize_ui_schema_into_map(property.schema(), &mut property_map);

        if !property_map.is_empty() {
            properties.insert(property.name().to_string(), Value::Object(property_map));
        }
    }

    map.extend(properties);

    if let Some(title) = schema.keys().and_then(|x| x.title()) {
        map.insert("ui:keys".to_string(), json!({ "ui:title": title }));
    }

    if !order.is_empty() {
        map.insert("ui:order".to_string(), json!(order));
    }
}

fn serialize_array_items(schema: &Schema, map: &mut Map<String, Value>) {
    if schema.items().is_empty() {
        return;
    }

    if schema.items().len() > 1 {
        // FIXME How it should look like?
        return;
    }

    let mut result: Map<String, Value> = Map::new();
    serialize_ui_schema_into_map(schema.items().first().unwrap(), &mut result);

    if !result.is_empty() {
        map.insert("items".to_string(), json!(result));
    }
}

fn serialize_ui_schema_into_map(schema: &Schema, map: &mut Map<String, Value>) {
    serialize_annotations(schema, map);
    serialize_properties(schema, map);
    serialize_widget(schema, map);
    serialize_ui_options(schema, map);
    serialize_array_items(schema, map);

    if schema.read_only() {
        map.insert("ui:readonly".to_string(), json!(true));
    }
}

fn serialize_as_ui_schema<O, E, S>(schema: &Schema, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    let mut result: Map<String, Value> = Map::new();
    serialize_ui_schema_into_map(schema, &mut result);

    for (k, v) in result.iter() {
        map.serialize_entry(k, v)?;
    }

    Ok(())
}
