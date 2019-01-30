use serde::ser::{Error, Serialize, SerializeMap, Serializer};

use crate::dsl::schema::Schema;
use crate::dsl::schema::SchemaList;
use crate::output::serialization::object_types::serialize_object_type;
use crate::output::serialization::object_types::serialize_keys_values;

pub fn serialize_schema_list<O, E, S>(schema_list: &SchemaList, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    map.serialize_entry("properties", &schema_list.all_as_map())?;

    let required = &schema_list.required_schema_names();
    if !required.is_empty() {
        map.serialize_entry("required", required)?;
    }

    map.serialize_entry("$$order", &schema_list.all_names())?;
    Ok(())
}

// FIXME: do not use trait implementation as it is hard to track where this is being called from
impl Serialize for Schema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        serialize_schema(&self, &mut map)?;
        map.end()
    }
}

pub fn serialize_schema<O, E, S>(schema: &Schema, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    for title in &schema.annotations.title {
        map.serialize_entry("title", &title)?;
    }

    serialize_object_type(&schema.object_type, map)?;

    if let Some(children) = &schema.children {
        serialize_schema_list(children, map)?;
    }

    if let Some(keysvalues) = &schema.dynamic {
        serialize_keys_values(keysvalues, map)?;
    }

    if let Some(formula) = &schema.formula {
        map.serialize_entry("$$formula", &formula)?;
    }
    Ok(())
}
