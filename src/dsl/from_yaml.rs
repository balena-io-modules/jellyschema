use crate::dsl::compiler::PropertyEntry;
use crate::dsl::compiler::PropertyList;
use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize_property_list<'de, D>(deserializer: D) -> Result<Option<PropertyList>, D::Error>
where
    D: Deserializer<'de>,
{
    let maybe_sequence: Option<serde_yaml::Sequence> = Option::deserialize(deserializer)?;
    let mut property_names = vec![];
    match maybe_sequence {
        Some(sequence) => {
            let list_of_maybe_entries = sequence.iter().map(|value| match value.as_mapping() {
                Some(mapping) => {
                    let (key, value) = match mapping.into_iter().next() {
                        Some(s) => s,
                        None => return Err(serde::de::Error::custom("cannot get first element of the sequence")),
                    };
                    let key: String = match serde_yaml::from_value(key.clone()) {
                        Ok(k) => k,
                        Err(e) => return Err(serde::de::Error::custom("cannot deserialize the key")),
                    };
                    let value = match serde_yaml::from_value(value.clone()) {
                        Ok(k) => k,
                        Err(e) => {
                            return Err(serde::de::Error::custom(format!(
                                "cannot deserialize the value {:?}",
                                e
                            )))
                        }
                    };
                    property_names.push(key.clone());
                    Ok(PropertyEntry {
                        name: key,
                        property: value,
                    })
                }
                None => Err(serde::de::Error::custom("")),
            });

            let list: Result<Vec<_>, D::Error> = list_of_maybe_entries.collect();
            let list = list?;

            Ok(Some(PropertyList {
                entries: list,
                property_names: property_names.clone(),
            }))
        }
        None => Ok(None),
    }
}
