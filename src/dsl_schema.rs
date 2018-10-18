use serde_derive::Deserialize;

pub type Schema = serde_yaml::Value;

#[derive(Deserialize)]
pub struct UIObject;

pub mod compiler {
    use serde_yaml::Value as YamlValue;

    pub struct Compiler {
        schema: YamlValue,
    }

    #[derive(Debug)]
    pub struct CompilationError;

    impl Compiler {
        pub fn new(schema: YamlValue) -> Self {
            Compiler { schema }
        }

        pub fn compile(self) -> Result<CompiledSchema, CompilationError> {
            let title = match self.schema["title"].as_str() {
                Some(title) => title,
                None => return Err(CompilationError),
            };
            Ok(CompiledSchema::with_title(&title))
        }
    }

    pub struct CompiledSchema {
        title: String,
    }

    impl CompiledSchema {
        pub fn empty() -> Self {
            CompiledSchema { title: "".to_string() }
        }

        pub fn with_title(title: &str) -> Self {
            CompiledSchema {
                title: title.to_string(),
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
        fn have_title() {
            let mut schema = Mapping::new();
            schema.insert("title".into(), "some title".into());
            let compiler = Compiler::new(serde_yaml::Value::from(schema));

            let compiled_schema = compiler.compile().unwrap();

            assert_eq!(compiled_schema.title(), "some title");
        }

    }

}

// TODO: add test for unsupported version number
