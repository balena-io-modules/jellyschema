use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Schema {
    version: u64,
    title: String,
}

#[derive(Deserialize)]
pub struct UIObject;

pub struct Compiler;

impl Compiler {
    pub fn new(schema: serde_yaml::Value) -> Self {
        Compiler {}
    }

    pub fn compile(self) -> CompiledSchema {
        CompiledSchema::new()
    }
}

pub struct CompiledSchema;
impl CompiledSchema {
    pub fn new() -> Self {
        CompiledSchema {}
    }
}
