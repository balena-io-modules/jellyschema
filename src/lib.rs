//! [balena] **c**onfiguration **dsl**
//!
//! A crate that provides facilities to:
//!
//! * transform configuration DSL into the JSON Schema & UI Object Schema with custom extensions
//! * parse configuration DSL
//!
//! # Versioning
//!
//! This crate is being actively developed and it does NOT follow [Semantic Versioning] yet.
//! It will follow semantic versioning when it reaches version 1.0.
//!
//! MINOR version changes denotes incompatible API changes and PATCH version changes denotes
//! both new functionality in a backwards-compatible manner and backwards-compatible bug fixes.
//!
//! # Examples
//!
//! ## Generate JSON Schema & UI Object
//!
//! ```
//! use jellyschema::generator::generate_json_ui_schema;
//! use jellyschema::schema::Schema;
//! use serde_yaml;
//!
//! let dsl = r#"
//!   version: 1
//!   properties:
//!     - name:
//!         type: string
//!         help: You should type your name here
//! "#;
//!
//! let input_schema: Schema = serde_yaml::from_str(dsl).unwrap();
//!
//! let (json_schema, ui_object) = generate_json_ui_schema(&input_schema);
//! ```
//!
//! [balena]: https://www.balena.io
//! [Semantic Versioning]: https://semver.org/
pub(crate) mod deref;
pub mod error;
pub mod schema;
pub mod validator;

pub mod generator;
#[cfg(all(target_arch = "wasm32", not(feature = "disable-wasm-bindings")))]
mod wasm;
