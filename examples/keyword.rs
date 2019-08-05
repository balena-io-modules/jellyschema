//! An example how to register custom keyword without validator.
//!
//! It's useful when you'd like to enhance Jelly Schema with custom
//! keyword, you'd like to validate that it's properly used within
//! the schema, but you do not want to validate data with it, because
//! you'll be using this keyword elsewhere / for something else.
//!
//! An example can be the Jelly Schema UI extension.

use serde_json::Value;

use jellyschema::keywords::{CompilationResult, Compiler, WalkContextExt};
use jellyschema::{Scope, ScopeBuilder, WalkContext};

struct Readable;

impl Compiler for Readable {
    fn compile(&self, schema: &Value, ctx: &WalkContext, _scope: &Scope) -> CompilationResult {
        // Allow missing 'readable' keyword
        let value = match schema.get("readable") {
            Some(x) => x,
            None => return Ok(None),
        };

        if value.is_boolean() {
            // Keyword exists, is of proper type, return without validators
            Ok(None)
        } else {
            // Keyword exists, is of a wrong type, return error
            ctx.compilation_error("readable", "expected boolean")
        }
    }
}

fn main() -> Result<(), String> {
    // Create scope with default keywords & data types and then add custom keyword
    let scope = ScopeBuilder::default().keyword(Box::new(Readable)).build();

    // Try without readable keyword
    assert!(scope.compile("type: string").is_ok());

    // Try with readable keyword
    assert!(scope.compile("type: string\nreadable: true").is_ok());

    // Try with readable keyword, but with a wrong type
    assert!(scope.compile("type: string\nreadable: foobar").is_err());

    Ok(())
}
