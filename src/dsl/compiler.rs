use crate::dsl::validation;
use serde::{Deserialize, Deserializer, Serializer};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json;

use std::collections::HashMap;

pub fn compile(schema: serde_yaml::Value) -> Result<SourceSchema, validation::Error> {
    println!("compiler start");
    let schema: SourceSchema = serde_yaml::from_value(schema)?;

    println!("compiler mid");
    if schema.version != 1 {
        return Err(validation::Error::invalid_version(schema.version));
    }

    Ok(schema)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ObjectType {
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "hostname")]
    Hostname
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Property {
    #[serde(rename = "type")]
    type_spec: Option<ObjectType>,
    title: Option<String>,
    help: Option<String>,
    warning: Option<String>,
    description: Option<String>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct PropertyEntry(pub String, pub Property);

#[derive(Clone, Debug, Deserialize)]
pub struct PropertyList(pub Vec<PropertyEntry>);

fn deserialize_property_list<'de, D>(deserializer: D) -> Result<Option<PropertyList>, D::Error>
where
    D: Deserializer<'de>,
{
    let maybe_sequence : Option<serde_yaml::Sequence> = Option::deserialize(deserializer)?;
    match maybe_sequence {
        Some(sequence) =>  {
            let list_of_maybe_entries =  sequence.iter().map( move |value| {
                match value.as_mapping() {
                    Some(mapping) => {

                        let (key, value) = match mapping.into_iter().next() {
                            Some(s) => s,
                            None => return Err(serde::de::Error::custom("cannot get first element of the sequence"))
                        };
                        let key = match serde_yaml::from_value(key.clone()) {
                            Ok(k) => k,
                            Err(e) => return Err(serde::de::Error::custom("cannot deserialize the key"))
                        };
                        let value = match serde_yaml::from_value(value.clone()) {
                            Ok(k) => k,
                            Err(e) => return Err(serde::de::Error::custom(format!("cannot deserialize the value {:?}", e)))
                        };

                        Ok(PropertyEntry(key, value))
                    },
                    None => Err(serde::de::Error::custom(""))
                }
            } );

            let list : Result<Vec<_>, D::Error> = list_of_maybe_entries.collect();
            let list = list?;

            Ok(Some(PropertyList(list)))
        },
        None => Ok(None)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SourceSchema {
    pub title: String,
    pub version: u64,
    #[serde( default, deserialize_with = "deserialize_property_list" )]
    pub properties: Option<PropertyList>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_yaml::Mapping;

    const SOME_TITLE: &str = "some title";

    #[test]
    fn pass_title_through() -> Result<(), validation::Error> {
        let schema = yaml_schema_with(SOME_TITLE, 1);

        assert_eq!(compile(schema)?.title, SOME_TITLE);
        Ok(())
    }

    #[test]
    // TODO: morph into property, so that the actual unsupported version is rand
    fn fail_on_unsupported_version() {
        let schema = yaml_schema_with(SOME_TITLE, 13);

        assert!(compile(schema).is_err());
    }

    #[test]
    fn fail_on_missing_version() {
        let mut schema = Mapping::new();
        schema.insert("title".into(), SOME_TITLE.into());
        let schema = serde_yaml::Value::Mapping(schema);

        assert!(compile(schema).is_err());
    }

    #[test]
    fn fail_on_missing_title() {
        let mut schema = Mapping::new();
        schema.insert("version".into(), 1.into());
        let schema = serde_yaml::Value::Mapping(schema);

        assert!(compile(schema).is_err());
    }

    fn yaml_schema_with(title: &str, version: u64) -> serde_yaml::Value {
        let mut schema = Mapping::new();
        schema.insert("title".into(), title.into());
        schema.insert("version".into(), version.into());
        serde_yaml::Value::Mapping(schema)
    }
}
