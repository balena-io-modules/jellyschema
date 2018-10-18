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
        pub fn with_message(message: &str) -> Self {
            CompilationError {
                message: message.to_string(),
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
                None => return Err(CompilationError::with_message("Missing title")),
            };

            let version = match self.schema["version"].as_u64() {
                Some(version) => version,
                None => return Err(CompilationError::with_message("Missing version")),
            };

            if version != 1 {
                return Err(CompilationError::with_message(&format!("Invalid version {}", version)));
            }

            Ok(CompiledSchema::with(&title, version))
        }
    }

    pub struct CompiledSchema {
        title: String,
        _version: u64,
    }

    impl CompiledSchema {
        pub fn empty() -> Self {
            CompiledSchema {
                title: "".to_string(),
                _version: 1,
            }
        }

        pub fn with(title: &str, version: u64) -> Self {
            CompiledSchema {
                title: title.to_string(),
                _version: version,
            }
        }

        pub fn title(&self) -> &str {
            &self.title
        }
    }

    #[cfg(test)]
    mod must {

        use super::*;
        use serde_yaml::Mapping;

        #[test]
        fn pass_title_through() {
            let mut schema = Mapping::new();
            schema.insert("title".into(), "some title".into());
            schema.insert("version".into(), 1.into());
            let compiler = Compiler::new(serde_yaml::Value::from(schema));

            let compiled_schema = compiler.compile().unwrap();

            assert_eq!(compiled_schema.title(), "some title");
        }

        #[test]
        fn fail_on_missing_version() {
            let mut schema = Mapping::new();
            schema.insert("title".into(), "some title".into());
            let compiler = Compiler::new(serde_yaml::Value::from(schema));

            let compilation_error = compiler.compile().err().unwrap();

            assert_eq!(compilation_error, CompilationError::with_message("Missing version"))
        }

        #[test]
        // TODO: morph into property, so that the actual unsupported version is rand
        fn fail_on_unsupported_version() {
            let mut schema = Mapping::new();
            schema.insert("title".into(), "some title".into());
            schema.insert("version".into(), 13.into());

            let compiler = Compiler::new(serde_yaml::Value::from(schema));

            let compilation_error = compiler.compile().err().unwrap();

            assert_eq!(compilation_error, CompilationError::with_message("Invalid version 13"))
        }

        #[test]
        fn fail_on_missing_title() {
            let mut schema = Mapping::new();
            schema.insert("version".into(), 1.into());
            let compiler = Compiler::new(serde_yaml::Value::from(schema));

            let compilation_error = compiler.compile().err().unwrap();

            assert_eq!(compilation_error, CompilationError::with_message("Missing title"))
        }
    }

}
