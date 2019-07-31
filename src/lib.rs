//! Jelly Schema
//!
//! # Example
//!
//! ```rust
//! use serde_json::{Value, json};
//! use jellyschema::Scope;
//!
//! // YAML schema
//! const SCHEMA: &str = "type: port";
//!
//! // Scope with default keywords & data types.
//! let scope = Scope::default();
//!
//! // Compile schema within the scope
//! let schema = scope.compile(SCHEMA).unwrap();
//!
//! // Validate data
//! assert!(schema.validate(&json!(10)).is_valid());
//! assert!(schema.validate(&json!(65536)).is_invalid());
//! assert!(schema.validate(&json!("foo")).is_invalid());
//! ```

// Keep it first so all macros are loaded upfront.
#[macro_use]
mod macros;

use std::borrow::Cow;

pub use serde_json::Value;

use crate::data_types::{BoxedDataType, DataTypeMap};
use crate::keywords::{BoxedCompiler, KeywordList};
use crate::path::{Path, PathItem};
use crate::validators::{BoxedValidator, ValidationState};

#[macro_use]
pub mod validators;
pub mod data_types;
pub mod generator;
pub mod keywords;
pub mod path;

#[cfg(all(target_arch = "wasm32", not(feature = "disable-wasm-bindings")))]
mod wasm;

/// Jelly Schema scope builder.
pub struct ScopeBuilder {
    keywords: KeywordList,
    data_types: DataTypeMap,
}

impl Default for ScopeBuilder {
    fn default() -> Self {
        ScopeBuilder::new().default_data_types().default_keywords()
    }
}

impl ScopeBuilder {
    /// Create new builder without keywords & data types.
    pub fn new() -> ScopeBuilder {
        ScopeBuilder {
            keywords: KeywordList::new(),
            data_types: DataTypeMap::new(),
        }
    }

    /// Build scope.
    pub fn build(self) -> Scope {
        Scope {
            keywords: self.keywords,
            data_types: self.data_types,
        }
    }

    /// Add all default data types.
    pub fn default_data_types(self) -> ScopeBuilder {
        let mut data_types = self.data_types;
        data_types.extend(data_types::default());
        ScopeBuilder {
            keywords: self.keywords,
            data_types,
        }
    }

    /// Add all default keywords.
    pub fn default_keywords(self) -> ScopeBuilder {
        let mut keywords = self.keywords;
        keywords.extend(keywords::default());
        ScopeBuilder {
            keywords,
            data_types: self.data_types,
        }
    }

    /// Add custom data type.
    ///
    /// Data type is **not** added if it already exists.
    ///
    /// # Arguments
    ///
    /// * `name` - Data type name.
    /// * `data_type` - Data type implementation.
    pub fn data_type<S>(self, name: S, data_type: BoxedDataType) -> ScopeBuilder
    where
        S: Into<String>,
    {
        let name = name.into();

        if self.data_types.contains_key(&name) {
            return self;
        }

        let mut data_types = self.data_types;
        data_types.insert(name, data_type);

        ScopeBuilder {
            keywords: self.keywords,
            data_types,
        }
    }

    /// Add custom keyword.
    ///
    /// # Arguments
    ///
    /// * `keyword` - Keyword implementation.
    pub fn keyword(self, keyword: BoxedCompiler) -> ScopeBuilder {
        let mut keywords = self.keywords;
        keywords.push(keyword);

        ScopeBuilder {
            keywords,
            data_types: self.data_types,
        }
    }
}

/// Jelly Schema scope.
pub struct Scope {
    /// Keyword compilers.
    pub(crate) keywords: KeywordList,
    /// Data types (except built-in ones).
    pub(crate) data_types: DataTypeMap,
}

impl Default for Scope {
    fn default() -> Self {
        ScopeBuilder::default().build()
    }
}

impl Scope {
    /// Compile schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - JellySchema raw value (YAML).
    pub fn compile(&self, schema: &str) -> Result<Schema, keywords::Error> {
        let schema: Value = serde_yaml::from_str(schema)?;
        self.compile_from_value(schema)
    }

    /// Compile schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - Deserialized JellySchema.
    pub fn compile_from_value(&self, schema: Value) -> Result<Schema, keywords::Error> {
        self.compile_from_value_in_context(schema, &WalkContext::new())
    }

    /// Compile schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - Deserialized JellySchema.
    /// * `ctx` - Walk context to start with.
    pub(crate) fn compile_from_value_in_context(
        &self,
        schema: Value,
        ctx: &WalkContext,
    ) -> Result<Schema, keywords::Error> {
        if !schema.is_object() {
            return Err(keywords::ErrorKind::CompileSchemaError(
                "$".to_string(),
                "",
                "Invalid schema, must be an object".to_string(),
            )
            .into());
        }
        let mut validators = vec![];

        for compiler in &self.keywords {
            if let Some(validator) = compiler.compile(&schema, ctx, self)? {
                validators.push(validator);
            }
        }

        Ok(Schema::new(schema, validators))
    }
}

/// Compiled Jelly Schema.
pub struct Schema {
    /// Deserialized schema (raw value).
    pub raw: Value,
    /// List of validators.
    pub(crate) validators: Vec<BoxedValidator>,
}

impl Schema {
    /// Create new schema.
    ///
    /// # Arguments
    ///
    /// * `raw` - Deserialized schema raw value.
    /// * `validators` - List of compiled validators.
    pub fn new(raw: Value, validators: Vec<BoxedValidator>) -> Schema {
        Schema { raw, validators }
    }

    /// Validate data with empty `WalkContext`.
    ///
    /// # Arguments
    ///
    /// * `data` - JSON data to validate.
    pub fn validate(&self, data: &Value) -> ValidationState {
        self.validate_in_context(data, &WalkContext::new())
    }

    /// Validate data within known `WalkContext`.
    ///
    /// # Arguments
    ///
    /// * `data` - JSON data to validate.
    /// * `ctx` - Walk context.
    pub fn validate_in_context(&self, data: &Value, ctx: &WalkContext) -> ValidationState {
        let mut state = ValidationState::new();

        for validator in &self.validators {
            state.append(validator.validate(data, ctx));
        }

        state
    }
}

/// Schema / data path.
#[derive(Debug, Clone)]
pub struct WalkContext<'a> {
    path: Cow<'a, Path>,
}

impl<'a> Default for WalkContext<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> WalkContext<'a> {
    /// Create new context with empty path.
    ///
    /// # Example
    ///
    /// ```rust
    /// use jellyschema::WalkContext;
    ///
    /// let ctx = WalkContext::new();
    /// assert_eq!(ctx.json_path(), "$".to_string());
    /// ```
    pub fn new() -> WalkContext<'a> {
        let path = Path::new();
        WalkContext { path: Cow::Owned(path) }
    }

    /// Push new path item at the end.
    ///
    /// # Example
    ///
    /// ```rust
    /// use jellyschema::WalkContext;
    ///
    /// let ctx = WalkContext::new().push("foo").push("bar").push(2);
    /// assert_eq!(ctx.json_path(), "$['foo']['bar'][2]".to_string());
    /// ```
    pub fn push<T>(&self, index: T) -> WalkContext<'a>
    where
        T: Into<PathItem>,
    {
        let mut path = self.path.to_owned();
        path.to_mut().push(index);
        WalkContext { path }
    }

    /// Get full JSON path.
    ///
    /// # Example
    ///
    /// ```rust
    /// use jellyschema::WalkContext;
    ///
    /// let ctx = WalkContext::new().push("foo").push(1);
    /// assert_eq!(ctx.json_path(), "$['foo'][1]".to_string());
    /// ```
    pub fn json_path(&self) -> String {
        format!("{}", self.path)
    }
}
