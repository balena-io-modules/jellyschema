//! This contains everything concerning parsing the DSL from yaml into `CompiledSchema` understood by `output::Generator`
//! Contains types representing DSL schema constructs and deserializers for them. Uses `serde_yaml` internally.
pub mod schema;
