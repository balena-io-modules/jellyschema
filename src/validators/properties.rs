use serde_json::Value;

use std::collections::{HashMap, HashSet};

use super::{ValidationState, Validator, WalkContextExt};
use crate::{Schema, WalkContext};

pub struct Properties {
    pub additional_properties: bool,
    pub keys_and_values: Option<(Schema, Schema)>,
    pub known_properties: HashMap<String, Schema>,
}

impl Validator for Properties {
    fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        if let Some(object) = data.as_object() {
            let mut validated_properties = HashSet::new();

            //
            // Iterate over object properties and validate them
            //
            for (key, value) in object.iter() {
                let ctx = ctx.push(key.as_str());

                validated_properties.insert(key);

                // Is it a know property?
                if let Some(schema) = self.known_properties.get(key) {
                    let state = schema.validate_in_context(value, &ctx);

                    if state.is_valid() {
                        continue;
                    }

                    return state;
                }

                // Unknown property, but maybe matches keys & values?
                if let Some((key_schema, value_schema)) = &self.keys_and_values {
                    let state = key_schema.validate_in_context(&Value::String(key.clone()), &ctx);

                    if state.is_valid() {
                        let state = value_schema.validate_in_context(value, &ctx);
                        if state.is_valid() {
                            continue;
                        }

                        return state;
                    }
                }

                // Unknown property, key doesn't match key schema, are additional properties allowed?
                if !self.additional_properties {
                    return ctx.validation_error("additionalProperties", "not allowed").into();
                }
            }

            //
            // Iterate over known properties and validate that they're optional
            //
            for (name, schema) in self.known_properties.iter() {
                if validated_properties.contains(name) {
                    continue;
                }

                let state = schema.validate_in_context(&Value::Null, &ctx.push(name.as_str()));

                if state.is_invalid() {
                    return state;
                }
            }
        }
        ValidationState::new()
    }
}
