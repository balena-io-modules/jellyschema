use std::collections::HashMap;
use std::string::ToString;

use serde::ser::{Error, SerializeMap};
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

use crate::schema::{PrimitiveType, Schema, UniqueItems};

// we output Draft 4 of the Json Schema specification because the downstream consumers
// of the JSON schema we produce fully support Draft 4, and not really Draft 7;
// in general most of the tools and libraries on the internet understand Draft 4 but have some problems with Draft 7
const SCHEMA_URL: &str = "http://json-schema.org/draft-04/schema#";

pub struct JsonSchema<'a> {
    schema_url: Option<&'static str>,
    schema: &'a Schema,
}

impl<'a> Serialize for JsonSchema<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        if let Some(url) = self.schema_url {
            map.serialize_entry("$schema", url)?;
        }

        serialize_as_json_schema(&self.schema, &mut map)?;
        map.end()
    }
}

impl<'a> JsonSchema<'a> {
    pub fn new(schema: &'a Schema) -> Self {
        JsonSchema {
            schema,
            schema_url: None,
        }
    }

    pub fn with_default_schema_url(schema: &'a Schema) -> Self {
        JsonSchema {
            schema,
            schema_url: Some(SCHEMA_URL),
        }
    }
}

fn serialize_type<O, E, S>(schema: &Schema, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    let primitive_type = schema.r#type().primitive_type();

    let (typ, additional_keywords): (&str, Value) = match primitive_type {
        PrimitiveType::Hostname => ("string", json!({"format": "hostname"})),
        PrimitiveType::Password => ("string", json!({"writeOnly": true})),
        PrimitiveType::DateTime => ("string", json!({"format": "date-time"})),
        PrimitiveType::Date => ("string", json!({"format": "date"})),
        PrimitiveType::Time => ("string", json!({"format": "time"})),
        PrimitiveType::IPv4 => ("string", json!({"format": "ipv4"})),
        PrimitiveType::IPv6 => ("string", json!({"format": "ipv6"})),
        PrimitiveType::Uri => ("string", json!({"format": "uri"})),
        PrimitiveType::Text => ("string", Value::Null),
        PrimitiveType::StringList => ("array", Value::Null),
        PrimitiveType::DNSMasqAddress => ("string", json!({"format": "dnsmasq-address"})),
        PrimitiveType::ChronyAddress => ("string", json!({"format": "chrony-address"})),
        PrimitiveType::IPTablesAddress => ("string", json!({"format": "iptables-address"})),
        PrimitiveType::Email => ("string", json!({"format": "email"})),
        PrimitiveType::Object => ("object", Value::Null),
        PrimitiveType::Array => ("array", Value::Null),
        PrimitiveType::String => ("string", Value::Null),
        PrimitiveType::Boolean => ("boolean", Value::Null),
        PrimitiveType::Integer => ("integer", Value::Null),
        PrimitiveType::Number => ("number", Value::Null),
        PrimitiveType::File => ("string", json!({"format": "data-url"})),
        PrimitiveType::Port => ("integer", {
            match (schema.min(), schema.max()) {
                (None, Some(_)) => json!({"minimum": 0}),
                (Some(_), None) => json!({"maximum": 65535}),
                (None, None) => json!({"minimum": 0, "maximum": 65535}),
                _ => Value::Null,
            }
        }),
    };

    map.serialize_entry("type", typ)?;

    if let Some(obj) = additional_keywords.as_object() {
        for (k, v) in obj.iter() {
            map.serialize_entry(k, v)?;
        }
    }

    Ok(())
}

fn serialize_annotations<O, E, S>(schema: &Schema, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let Some(title) = schema.title() {
        map.serialize_entry("title", title)?;
    }

    if let Some(description) = schema.description() {
        map.serialize_entry("description", description)?;
    }

    Ok(())
}

fn serialize_array_keywords<O, E, S>(schema: &Schema, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let Some(min_items) = schema.min_items() {
        map.serialize_entry("minItems", &min_items)?;
    }

    if let Some(max_items) = schema.max_items() {
        map.serialize_entry("maxItems", &max_items)?;
    }

    match schema.unique_items() {
        UniqueItems::Boolean(value) if *value => map.serialize_entry("uniqueItems", value)?,
        UniqueItems::Paths(paths) => map.serialize_entry("$$uniqueItemProperties", paths)?,
        _ => {}
    };

    let items_count = schema.items().len();
    match items_count {
        0 => {}
        1 => map.serialize_entry("items", &JsonSchema::new(schema.items().first().unwrap()))?,
        _ => {
            let json_schemas: Vec<JsonSchema> = schema.items().iter().map(JsonSchema::new).collect();
            map.serialize_entry("items", &json!({ "oneOf": json_schemas }))?;
        }
    };

    Ok(())
}

fn serialize_object_keywords<O, E, S>(schema: &Schema, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let PrimitiveType::Object = schema.r#type().primitive_type() {
        map.serialize_entry("additionalProperties", &schema.additional_properties())?;
    }

    if !schema.properties().is_empty() {
        let mut required = vec![];
        let mut order = vec![];

        let mut properties = HashMap::<&str, JsonSchema>::new();

        for property in schema.properties() {
            if property.schema().r#type().is_required() {
                required.push(property.name());
            }
            order.push(property.name());

            properties.insert(property.name(), JsonSchema::new(property.schema()));
        }

        if !required.is_empty() {
            map.serialize_entry("required", &required)?;
        }

        if !order.is_empty() {
            map.serialize_entry("$$order", &order)?;
        }

        if !properties.is_empty() {
            map.serialize_entry("properties", &properties)?;
        }
    }

    match (schema.keys(), schema.values()) {
        (Some(keys), Some(values)) if keys.pattern().is_some() => map.serialize_entry(
            "patternProperties",
            &json!({ keys.pattern().unwrap().to_string(): JsonSchema::new(values) }),
        )?,
        _ => {}
    };

    Ok(())
}

fn serialize_number_keywords<O, E, S>(schema: &Schema, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let Some(multiple_of) = schema.multiple_of() {
        map.serialize_entry("multipleOf", &multiple_of)?;
    }

    match (schema.max(), schema.exclusive_max()) {
        (_, Some(max)) => {
            map.serialize_entry("exclusiveMaximum", &true)?;
            map.serialize_entry("maximum", &max)?;
        }
        (Some(max), None) => {
            map.serialize_entry("maximum", &max)?;
        }
        _ => {}
    };

    match (schema.min(), schema.exclusive_min()) {
        (_, Some(min)) => {
            map.serialize_entry("exclusiveMinimum", &true)?;
            map.serialize_entry("minimum", &min)?;
        }
        (Some(min), None) => {
            map.serialize_entry("minimum", &min)?;
        }
        _ => {}
    };

    Ok(())
}

fn serialize_string_keywords<O, E, S>(schema: &Schema, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if let Some(max_length) = schema.max_length() {
        map.serialize_entry("maxLength", &max_length)?;
    }

    if let Some(min_length) = schema.min_length() {
        map.serialize_entry("minLength", &min_length)?;
    }

    if let Some(pattern) = schema.pattern() {
        map.serialize_entry("pattern", &pattern.to_string())?;
    }

    Ok(())
}

fn serialize_as_json_schema<O, E, S>(schema: &Schema, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    serialize_annotations(schema, map)?;
    serialize_array_keywords(schema, map)?;
    serialize_object_keywords(schema, map)?;
    serialize_number_keywords(schema, map)?;
    serialize_string_keywords(schema, map)?;

    serialize_type(schema, map)?;

    if let Some(version) = schema.version() {
        map.serialize_entry("$$version", &version)?;
    }

    if let Some(cons) = schema.r#const() {
        map.serialize_entry("enum", &vec![cons])?;
    }

    if let Some(def) = schema.default() {
        map.serialize_entry("default", def)?;
    }

    if let Some(formula) = schema.formula() {
        map.serialize_entry("$$formula", formula)?;
    }

    if schema.read_only() {
        map.serialize_entry("readOnly", &true)?;
    }

    if schema.write_only() {
        map.serialize_entry("writeOnly", &true)?;
    }

    let values: Vec<Value> = schema
        .r#enum()
        .iter()
        .map(|x| json!({ "title": x.title(), "enum": [ x.value() ]}))
        .collect();
    if !values.is_empty() {
        map.serialize_entry("oneOf", &values)?;
    }

    Ok(())
}
