use std::collections::HashMap;

use serde_json::Value;

use super::{CompilationResult, Compiler, Error, WalkContextExt};
use crate::validators;
use crate::{Schema, Scope, WalkContext};

pub struct Properties;

fn get_additional_properties(schema: &Value, ctx: &WalkContext) -> Result<bool, Error> {
    match schema.get("additionalProperties") {
        Some(v) => {
            if let Some(value) = v.as_bool() {
                Ok(value)
            } else {
                ctx.compilation_error("additionalProperties", "expected boolean")
            }
        }
        None => Ok(false),
    }
}

fn get_known_properties(schema: &Value, ctx: &WalkContext, scope: &Scope) -> Result<HashMap<String, Schema>, Error> {
    let mut m = HashMap::new();

    if let Some(properties) = schema.get("properties") {
        let array = match properties.as_array() {
            Some(x) => x,
            None => return ctx.compilation_error("properties", "expected array of properties"),
        };

        let ctx = ctx.push("properties");

        for (idx, value) in array.iter().enumerate() {
            let ctx = ctx.push(idx);

            let object = match value.as_object() {
                Some(x) => x,
                None => return ctx.compilation_error("properties", "expected schema object"),
            };

            if object.len() != 1 {
                return ctx.compilation_error("properties", "expected one schema object");
            }

            let (name, schema) = object.iter().next().unwrap();

            let schema = scope.compile_from_value_in_context(schema.clone(), &ctx)?;

            m.insert(name.clone(), schema);
        }
    }

    Ok(m)
}

fn get_keys_and_values(schema: &Value, ctx: &WalkContext, scope: &Scope) -> Result<Option<(Schema, Schema)>, Error> {
    let maybe_keys = schema.get("keys");
    let maybe_values = schema.get("values");

    if maybe_keys.is_none() && maybe_values.is_none() {
        return Ok(None);
    }

    if maybe_keys.is_none() || maybe_values.is_none() {
        if maybe_keys.is_none() {
            return ctx.compilation_error("keys", "expected schema object");
        }

        if maybe_values.is_none() {
            return ctx.compilation_error("values", "expected schema object");
        }
    }

    let keys_ctx = ctx.push("keys");
    let keys_schema = scope.compile_from_value_in_context(maybe_keys.unwrap().clone(), &keys_ctx)?;

    if keys_schema.raw.get("type").and_then(|x| x.as_str()) != Some("string") {
        return keys_ctx.compilation_error("type", "expected type 'string'");
    }

    if keys_schema.raw.get("pattern").is_none() {
        return keys_ctx.compilation_error("pattern", "expected regular expression");
    }

    let values_ctx = ctx.push("values");
    let values_schema = scope.compile_from_value_in_context(maybe_values.unwrap().clone(), &values_ctx)?;

    Ok(Some((keys_schema, values_schema)))
}

impl Compiler for Properties {
    fn compile(&self, schema: &Value, ctx: &WalkContext, scope: &Scope) -> CompilationResult {
        let additional_properties = get_additional_properties(schema, ctx)?;
        let known_properties = get_known_properties(schema, ctx, scope)?;
        let keys_and_values = get_keys_and_values(schema, ctx, scope)?;

        Ok(Some(Box::new(validators::properties::Properties {
            additional_properties,
            keys_and_values,
            known_properties,
        })))
    }
}
