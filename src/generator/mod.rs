//! A module containing output generator for the JSON Schema & UI Object
use serde_json::Value;

use crate::{
    generator::serialization::{JsonSchema, UiSchema},
    schema::Schema,
};

mod serialization;

fn generate_json_schema(schema: &Schema) -> Value {
    let json_schema: JsonSchema = JsonSchema::with_default_schema_url(schema);
    serde_json::to_value(json_schema).expect("Internal error: inconsistent schema: json schema")
}

fn generate_ui_schema(schema: &Schema) -> Value {
    let ui_schema: UiSchema = UiSchema::new(schema);
    serde_json::to_value(ui_schema).expect("Internal error: inconsistent schema: ui schema")
}

pub fn generate_json_ui_schema(schema: &Schema) -> (Value, Value) {
    (generate_json_schema(schema), generate_ui_schema(schema))
}
