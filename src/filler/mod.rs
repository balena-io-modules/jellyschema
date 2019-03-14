//! A module containing default values filler.
use serde_json::{json, Value};

use crate::schema::{PrimitiveType, Schema};

// Recursively check if the object is empty
//
// If the child object is empty, parent object is considered as empty.
fn is_empty_object(value: &Value) -> bool {
    match value {
        Value::Object(map) => map.values().all(|x| is_empty_object(x)),
        _ => false,
    }
}

fn fill_defaults(schema: &Schema, data: &mut Value, include_optional: bool) {
    match (schema.r#type().primitive_type(), schema.r#type().is_required()) {
        (PrimitiveType::Object, _) => {
            if data.is_null() {
                std::mem::replace(data, json!({}));
            }

            if let Some(data) = data.as_object_mut() {
                for property in schema.properties() {
                    let name = property.name();

                    if let Some(mut value) = data.get_mut(name) {
                        fill_defaults(property.schema(), &mut value, include_optional);
                    } else {
                        // Fill defaults, but if the resulting object is empty, do not include it
                        let mut value = Value::Null;
                        fill_defaults(property.schema(), &mut value, include_optional);
                        if !value.is_null() && !is_empty_object(&value) {
                            data.insert(name.to_string(), value);
                        }
                    }
                }
            }
        }
        (PrimitiveType::Array, _) => {
            if data.is_array() && schema.items().len() == 1 {
                // What we should do in case of multiple schemas? Partial object match?
                let schema = schema.items().first().unwrap();

                for mut item in data.as_array_mut().unwrap() {
                    fill_defaults(&schema, &mut item, include_optional);
                }
            }
        }
        (_, required) => {
            if let Some(default_value) = schema.r#default() {
                if data.is_null() && (include_optional || required) {
                    std::mem::replace(data, default_value.clone());
                }
            }
        }
    };
}

/// Fill default values from the schema
///
/// # Arguments
///
/// * `schema` - JellySchema
/// * `data` - JSON value to start with
/// * `include_optional` - if `false` only required properties are filled
pub fn fill_default_values(schema: &Schema, data: &mut Value, include_optional: bool) {
    fill_defaults(schema, data, include_optional);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fill_all(schema: &str, input: Value) -> Value {
        let mut input = input;
        fill_default_values(&schema.parse::<Schema>().unwrap(), &mut input, true);
        input
    }

    fn fill_required(schema: &str, input: Value) -> Value {
        let mut input = input;
        fill_default_values(&schema.parse::<Schema>().unwrap(), &mut input, false);
        input
    }

    #[test]
    fn fill_nested_required() {
        let schema = r##"
            properties:
                - foo:
                    properties:
                        - bar:
                            type: string
                            default: baz
        "##;
        let input = Value::Null;
        let result = json!({"foo": {"bar": "baz"}});
        assert_eq!(fill_required(schema, input), result);
    }

    #[test]
    fn ignore_nested_optional() {
        let schema = r##"
            properties:
                - foo:
                    properties:
                        - bar:
                            type: string?
                            default: baz
        "##;
        let input = Value::Null;
        let result = json!({});
        assert_eq!(fill_required(schema, input), result);
    }

    #[test]
    fn fill_nested_optional() {
        let schema = r##"
            properties:
                - foo:
                    properties:
                        - bar:
                            type: string?
                            default: baz
        "##;
        let input = Value::Null;
        let result = json!({"foo": {"bar": "baz"}});
        assert_eq!(fill_all(schema, input), result);
    }

    #[test]
    fn do_not_remove_empty_nested_objects() {
        let schema = r##"
            properties:
                - foo:
                    properties:
                        - bar:
                            type: string?
                            default: baz
        "##;
        let input = json!({"foo": {}});
        let result = json!({"foo": {}});
        assert_eq!(fill_required(schema, input), result);
    }

    #[test]
    fn fill_required_array_item_properties() {
        let schema = r##"
            type: array
            items:
                properties:
                    - foo:
                        type: string
                        default: bar
        "##;
        let input = json!([{}, {"foo": "baz"}, {"bar": "baz"}]);
        let result = json!([{"foo": "bar"}, {"foo": "baz"}, {"foo": "bar", "bar": "baz"}]);
        assert_eq!(fill_required(schema, input), result);
    }

    #[test]
    fn fill_required_root_string() {
        let schema = r##"
            type: string
            default: foo
        "##;
        let input = json!(null);
        let result = json!("foo");
        assert_eq!(fill_required(schema, input), result);
    }

    #[test]
    fn object_emptiness() {
        assert!(!is_empty_object(&json!("foo")));
        assert!(!is_empty_object(&json!(123)));
        assert!(!is_empty_object(&json!(true)));
        assert!(!is_empty_object(&json!(false)));
        assert!(is_empty_object(&json!({})));
        assert!(!is_empty_object(&json!([])));
        assert!(!is_empty_object(&json!({"foo": {"bar": "baz"}})));
        assert!(!is_empty_object(&json!({"foo": "bar"})));
        assert!(is_empty_object(&json!({"foo": {"bar": {}}})));
    }
}
