use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Schema {
    version: u64,
    title: String
}

pub struct Compiler;

impl Compiler {
    pub fn new(schema: Schema) -> Self {
        Compiler{}
    }

    pub fn compile(self) -> CompiledSchema {
       CompiledSchema::new()
    }
}

pub struct CompiledSchema;
impl CompiledSchema {
    pub fn new() -> Self {
        CompiledSchema{}
    }
}

pub struct Generator;

impl Generator {
    pub fn new(compiled_schema: CompiledSchema) -> Self {
        Generator{}
    }

    pub fn generate(self) -> (String, String) {
        ("".to_string(), "".to_string())
    }
}


