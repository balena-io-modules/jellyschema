//! This is is a Rust (edition 2018) library that is intended to be used as a translation mechanism between the [configuration DSL](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md) and the `JSON + UI Schema` world.
//! The is not production ready yet, however it supports some real-life cases already.
//! The plan is to have this library be available to be used from Rust code but also as a Node module to be used from Node and in-browser code, through WASM.
//!
//! # Examples
//!
//! ```
//! use balena_cdsl::output::generator::Generator;
//!
//! let input_schema : serde_yaml::Value = serde_yaml::from_str(
//! include_str!("../tests/data/schema/empty/input-schema.yml")).unwrap();
//!
//! let (json_schema, ui_object) = Generator::with(input_schema).unwrap().generate();
//! ```
pub mod dsl;
pub mod output;
#[cfg(target_arch = "wasm32")]
mod wasm;
