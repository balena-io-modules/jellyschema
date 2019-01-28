use regex::Regex;
use serde::de::Error;
use serde_yaml::Mapping;
use serde_yaml::Value;

use crate::dsl::schema::deserialization::deserialize_schema;
use crate::dsl::schema::KeysSchema;
use crate::dsl::schema::KeysValues;

pub fn keys_values<E>(yaml_mapping: &Mapping) -> Result<Option<Box<KeysValues>>, E>
where
    E: Error,
{
    let key = yaml_mapping.get(&Value::from("keys"));
    let value = yaml_mapping.get(&Value::from("values"));

    if key.is_none() && value.is_none() {
        return Ok(None);
    }

    if key.is_some() && value.is_none() {
        return Err(Error::custom("need `values` when specifying a `keys`"));
    }

    if key.is_none() && value.is_some() {
        return Err(Error::custom("need `keys` when specifying a `values`"));
    }

    let keys = deserialize_keys_schema(key.unwrap())?;
    let values = deserialize_schema(value.unwrap())?;

    Ok(Some(Box::new(KeysValues { keys, values })))
}

fn deserialize_keys_schema<E>(value: &Value) -> Result<KeysSchema, E>
where
    E: Error,
{
    let yaml_mapping = value
        .as_mapping()
        .ok_or_else(|| Error::custom(format!("schema is not a yaml mapping - {:#?}", value)))?;

    let pattern = yaml_mapping.get(&Value::from("pattern"));
    let title = yaml_mapping.get(&Value::from("title"));
    let type_spec = yaml_mapping.get(&Value::from("type"));

    if type_spec.is_none() {
        return Err(Error::custom("`keys` must have a `type` specified"));
    }

    let type_spec = type_spec.unwrap().as_str();
    if type_spec.is_none() {
        return Err(Error::custom("`keys` must have `type` specified as string"));
    }

    let title = match title {
        None => Ok(None),
        Some(title) => match title.as_str() {
            None => Err(Error::custom("`title` must be a string")),
            Some(title) => Ok(Some(title.to_string())),
        },
    }?;

    if pattern.is_none() {
        return Err(Error::custom("`keys` must have a `pattern`"));
    }

    let pattern = pattern.unwrap().as_str();
    if pattern.is_none() {
        return Err(Error::custom("`pattern` must be a string"));
    }

    let pattern =
        Regex::new(pattern.unwrap()).map_err(|e| Error::custom(format!("`pattern` is not a regex - {:#?}", e)))?;

    Ok(KeysSchema { pattern, title })
}
