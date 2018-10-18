use serde_derive::Deserialize;

pub type Schema = serde_yaml::Value;

#[derive(Deserialize)]
pub struct UIObject;

pub mod compiler {
    use serde_yaml::Value as YamlValue;

    pub struct Compiler {
        schema: YamlValue,
    }

    #[derive(Debug, PartialEq)]
    pub struct CompilationError {
        message: String,
    }

    impl CompilationError {
        pub fn missing_title() -> Self {
            CompilationError {
                message: "Missing title".to_string(),
            }
        }

        pub fn missing_version() -> Self {
            CompilationError {
                message: "Missing version".to_string(),
            }
        }

        pub fn invalid_version(version: u64) -> Self {
            CompilationError {
                message: format!("Invalid version {}", version),
            }
        }
    }

    impl Compiler {
        pub fn new(schema: YamlValue) -> Self {
            Compiler { schema }
        }

        // TODO: extract validator from the method's code
        pub fn compile(self) -> Result<CompiledSchema, CompilationError> {
            let title = match self.schema["title"].as_str() {
                Some(title) => title,
                None => return Err(CompilationError::missing_title()),
            };

            let version = match self.schema["version"].as_u64() {
                Some(version) => version,
                None => return Err(CompilationError::missing_version()),
            };

            if version != 1 {
                return Err(CompilationError::invalid_version(version));
            }

            Ok(CompiledSchema::with(&title, version))
        }
    }

    pub struct CompiledSchema {
        title: String,
        version: u64,
    }

    impl CompiledSchema {
        pub fn empty() -> Self {
            CompiledSchema {
                title: "".to_string(),
                version: 1,
            }
        }

        pub fn with(title: &str, version: u64) -> Self {
            CompiledSchema {
                title: title.to_string(),
                version,
            }
        }

        pub fn title(&self) -> &str {
            &self.title
        }

        pub fn version(&self) -> u64 {
            self.version
        }
    }

    #[cfg(test)]
    mod must {

        use super::*;
        use serde_yaml::Mapping;

        const SOME_TITLE: &str = "some title";

        #[test]
        fn pass_title_through() {
            let schema = yaml_schema_with(SOME_TITLE, 1);
            let compiler = Compiler::new(serde_yaml::Value::from(schema));

            let compiled_schema = compiler.compile().unwrap();

            assert_eq!(compiled_schema.title(), SOME_TITLE);
        }

        #[test]
        fn fail_on_missing_version() {
            let mut schema = Mapping::new();
            schema.insert("title".into(), "some title".into());
            let compiler = Compiler::new(serde_yaml::Value::from(schema));

            let compilation_error = compiler.compile().err().unwrap();

            assert_eq!(compilation_error, CompilationError::missing_version());
        }

        #[test]
        // TODO: morph into property, so that the actual unsupported version is rand
        fn fail_on_unsupported_version() {
            let schema = yaml_schema_with(SOME_TITLE, 13);

            let compiler = Compiler::new(serde_yaml::Value::from(schema));

            let compilation_error = compiler.compile().err().unwrap();

            assert_eq!(compilation_error, CompilationError::invalid_version(13));
        }

        #[test]
        fn fail_on_missing_title() {
            let mut schema = Mapping::new();
            schema.insert("version".into(), 1.into());
            let compiler = Compiler::new(serde_yaml::Value::from(schema));

            let compilation_error = compiler.compile().err().unwrap();

            assert_eq!(compilation_error, CompilationError::missing_title());
        }

        fn yaml_schema_with(title: &str, version: u64) -> Mapping {
            let mut schema = Mapping::new();
            schema.insert("title".into(), title.into());
            schema.insert("version".into(), version.into());
            schema
        }
    }

}
