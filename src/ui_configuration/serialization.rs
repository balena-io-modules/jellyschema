use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};
use crate::dsl::compiler::PropertyList;
use crate::dsl::compiler::PropertyEntry;
use std::collections::HashMap;

pub fn serialize_property_list<S>(property_list: &Option<PropertyList>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    match property_list {
        Some(list) => {
            let map : HashMap<String, PropertyEntry>= HashMap::new();
            let mut map = serializer.serialize_map(Some(map.iter().count()))?;
            for entry in list.clone().entries {
                map.serialize_entry(&entry.0, &entry.1);
            }
            map.end()
        },
        None => serializer.serialize_none()
    }
}
