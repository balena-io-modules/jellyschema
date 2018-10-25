use crate::dsl::validation;
use crate::dsl::validation::validate;
use crate::dsl::validation::ValidatedSchema;
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub fn compile(schema: serde_yaml::Value) -> Result<ValidatedSchema, validation::Error> {
    let schema: SourceSchema = serde_yaml::from_value(schema)?;
    let validated_schema = validate(schema)?;
    Ok(validated_schema)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectType {
    Object,
    Hostname,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub type_spec: Option<ObjectType>,
    pub title: Option<String>,
    pub help: Option<String>,
    pub warning: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PropertyEntry {
    pub name: String,
    pub property: Property,
}

#[derive(Clone, Debug)]
pub struct PropertyList {
    pub property_names: Vec<String>,
    pub entries: Vec<PropertyEntry>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SourceSchema {
    pub title: String,
    pub version: u64,
    #[serde(
        default,
        deserialize_with = "crate::dsl::from_yaml::deserialize_property_list"
    )]
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
