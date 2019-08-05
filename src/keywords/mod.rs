//! Keyword compilers.
use error_chain::*;
use serde_json::Value;

use crate::validators::BoxedValidator;
use crate::{Scope, WalkContext};

mod addable;
mod collapsed;
mod collapsible;
mod const_;
mod description;
mod enum_;
mod exclusive_max;
mod exclusive_min;
mod generate;
mod help;
mod hidden;
mod items;
mod max;
mod max_items;
mod max_length;
mod min;
mod min_items;
mod min_length;
mod multiple_of;
mod orderable;
mod pattern;
mod placeholder;
mod properties;
mod read_only;
mod removable;
mod title;
mod type_;
mod unique_items;
mod version;
mod warning;
mod write_only;

/// Compilation result.
pub type CompilationResult = Result<Option<BoxedValidator>>;

/// List of compiler trait objects.
pub type KeywordList = Vec<BoxedCompiler>;

/// Compiler trait object.
pub type BoxedCompiler = Box<dyn Compiler>;

/// Compiler interface.
pub trait Compiler {
    /// Compile schema keyword.
    ///
    /// # Arguments
    ///
    /// * `schema` - Deserialized schema.
    /// * `ctx` - Path in the whole schema.
    /// * `scope` - Jelly Schema scope.
    fn compile(&self, schema: &Value, ctx: &WalkContext, scope: &Scope) -> CompilationResult;
}

/// Compilation helpers.
pub trait WalkContextExt {
    /// Helper to create compilation error.
    fn compilation_error<S: Into<String>, T>(&self, keyword: &'static str, message: S) -> Result<T>;
}

impl WalkContextExt for WalkContext {
    fn compilation_error<S: Into<String>, T>(&self, keyword: &'static str, message: S) -> Result<T> {
        Err(ErrorKind::CompileSchemaError(self.json_path(), keyword, message.into()).into())
    }
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {}

    foreign_links {
        serde_yaml::Error, DeserializeSchemaError, "Schema (YAML) deserialization error";
    }

    errors {
        CompileSchemaError(path: String, keyword: &'static str, message: String) {
            description("Unable to compile schema")
            display("Unable to compile schema at '{}', keyword '{}', message '{}'", path, keyword, message)
        }
    }
}

pub(crate) fn default() -> KeywordList {
    let mut v = KeywordList::new();

    // Common

    v.push(Box::new(const_::Const));
    v.push(Box::new(type_::Type));
    v.push(Box::new(enum_::Enum));
    v.push(Box::new(generate::Generate));

    // Number

    v.push(Box::new(min::Min));
    v.push(Box::new(max::Max));
    v.push(Box::new(exclusive_max::ExclusiveMax));
    v.push(Box::new(exclusive_min::ExclusiveMin));
    v.push(Box::new(multiple_of::MultipleOf));

    // String

    v.push(Box::new(max_length::MaxLength));
    v.push(Box::new(min_length::MinLength));
    v.push(Box::new(pattern::Pattern));

    // Array

    v.push(Box::new(max_items::MaxItems));
    v.push(Box::new(min_items::MinItems));
    v.push(Box::new(items::Items));
    v.push(Box::new(unique_items::UniqueItems));

    // Object

    // Handles keys, values, additionalProperties & properties
    v.push(Box::new(properties::Properties));

    // Annotations

    v.push(Box::new(title::Title));
    v.push(Box::new(description::Description));
    v.push(Box::new(help::Help));
    v.push(Box::new(warning::Warning));
    v.push(Box::new(placeholder::Placeholder));

    v.push(Box::new(hidden::Hidden));

    v.push(Box::new(collapsed::Collapsed));
    v.push(Box::new(collapsible::Collapsible));

    v.push(Box::new(read_only::ReadOnly));
    v.push(Box::new(write_only::WriteOnly));

    v.push(Box::new(addable::Addable));
    v.push(Box::new(removable::Removable));
    v.push(Box::new(orderable::Orderable));

    // Meta

    v.push(Box::new(version::Version));

    v
}
