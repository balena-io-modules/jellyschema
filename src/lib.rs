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
//! use balena_cdsl::output::generator::Generator;
//! use serde_yaml::*;
//!
//! let dsl = r#"
//!   version: 1
//!   properties:
//!     - name:
//!         type: string
//!         help: You should type your name here
//! "#;
//!
//! let input_schema: serde_yaml::Value = serde_yaml::from_str(dsl).unwrap();
//!
//! let (json_schema, ui_object) = Generator::with(input_schema).unwrap().generate();
//! ```
//!
//! [balena]: https://www.balena.io
//! [Semantic Versioning]: https://semver.org/
pub mod dsl;
pub mod output;
#[cfg(target_arch = "wasm32")]
mod wasm;
