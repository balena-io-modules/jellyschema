use crate::dsl::validation;
use serde_derive::Deserialize;

pub fn compile(schema: serde_yaml::Value) -> Result<CompiledSchema, validation::Error> {
    let schema: CompiledSchema = serde_yaml::from_value(schema)?;

    if schema.version != 1 {
        return Err(validation::Error::invalid_version(schema.version));
    }

    Ok(schema)
}

#[derive(Clone, Deserialize)]
pub struct CompiledSchema {
    pub title: String,
    pub version: u64,
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
