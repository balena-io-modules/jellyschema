//! An example how to register custom keyword with validator.
//!
//! We add `odd` keyword to the schema, which checks if the provided
//! numbers are odd or even (depends on the `odd` keyword value).

use serde_json::{json, Value};

use jellyschema::{Scope, ScopeBuilder, WalkContext};

// Odd keyword compiler
mod keyword {
    use super::*;
    use jellyschema::keywords::{CompilationResult, Compiler, WalkContextExt};

    pub struct Odd;

    impl Compiler for Odd {
        fn compile(&self, schema: &Value, ctx: &WalkContext, _scope: &Scope) -> CompilationResult {
            // Allow missing 'odd' keyword
            let value = match schema.get("odd") {
                Some(x) => x,
                None => return Ok(None),
            };

            if let Some(odd) = value.as_bool() {
                // Keyword exists, is of proper type, return validator
                Ok(Some(Box::new(validator::Odd { odd })))
            } else {
                // Keyword exists, is of a wrong type, return error
                ctx.compilation_error("odd", "expected boolean")
            }
        }
    }
}

// Odd keyword validator
mod validator {
    use super::*;
    use jellyschema::validators::{ValidationState, Validator, WalkContextExt};

    pub struct Odd {
        pub odd: bool,
    }

    impl Validator for Odd {
        fn validate(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
            if let Some(number) = data.as_u64() {
                if self.odd && number % 2 == 0 {
                    return ctx
                        .validation_error("odd", format!("'{}' is not an odd number", number))
                        .into();
                }

                if !self.odd && number % 2 != 0 {
                    return ctx
                        .validation_error("odd", format!("'{}' is not an even number", number))
                        .into();
                }
            }

            ValidationState::new()
        }
    }

}

// Schema says that we'd like to have an integer which must be odd
const SCHEMA: &str = r##"
type: integer
odd: true
"##;

fn main() -> Result<(), String> {
    // Create scope with default keywords & data types and then add custom keyword
    let scope = ScopeBuilder::default().keyword(Box::new(keyword::Odd)).build();

    // Compile schema with enhanced scope
    let schema = scope
        .compile(SCHEMA)
        .map_err(|e| format!("Unable to compile schema: {}", e))?;

    // Try float - invalid
    assert!(schema.validate(&json!(11.3)).is_invalid());

    // Try even integer - invalid
    assert!(schema.validate(&json!(10)).is_invalid());

    // Try odd integer - valid
    assert!(schema.validate(&json!(11)).is_valid());

    Ok(())
}
