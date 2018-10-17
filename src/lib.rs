pub struct Compiler;

impl Compiler {
    pub fn new(schema_text: &str) -> Self {
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


