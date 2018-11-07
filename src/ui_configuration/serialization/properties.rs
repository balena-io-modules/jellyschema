use crate::dsl::object_types::ObjectType;
use crate::dsl::schema::{Property, PropertyList};
use crate::ui_configuration::serialization::object_types::object_type_name;
use crate::ui_configuration::serialization::object_types::serialize_object_type;
use serde::ser::{Error, Serialize, SerializeMap, Serializer};
use std::collections::HashMap;

pub fn serialize_property_list<O, E, S>(property_list: &PropertyList, map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if !property_list.entries.is_empty() {
        let mut properties_map = HashMap::new();
        for property in &property_list.entries {
            properties_map.insert(&property.name, &property.property);
        }
        map.serialize_entry("properties", &properties_map)?;
    };

    let required = &property_list.required_property_names();
    if !required.is_empty() {
        map.serialize_entry("required", required)?;
    }

    let names = &property_list.property_names();
    if !names.is_empty() {
        map.serialize_entry("$$order", names)?;
    }
    Ok(())
}

impl Serialize for Property {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        for title in &self.display_information.title {
            map.serialize_entry("title", &title)?;
        }

        if let Some(types) = &self.types {
            if types.len() == 1 {
                serialize_object_type(types[0].inner(), &mut map)?;
            }
            if types.len() > 1 {
                serialize_type_array(types, &mut map)?;
            }
        }

        let property_list = &self.property_list;
        if let Some(properties) = property_list {
            serialize_property_list(properties, &mut map)?;
        }

        map.end()
    }
}

fn serialize_type_array<O, E, S>(types: &[ObjectType], map: &mut S) -> Result<(), E>
where
    E: Error,
    S: SerializeMap<Ok = O, Error = E>,
{
    if types.iter().any(|def| def.inner().has_bounds()) {
        return Err(Error::custom(
            "cannot have type bounds when specifying multiple types per property",
        ));
    }

    let type_names: Vec<_> = types.iter().map(|def| object_type_name(def.inner())).collect();
    map.serialize_entry("type", &type_names)?;
    Ok(())
}
