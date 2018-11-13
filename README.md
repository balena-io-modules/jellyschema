# Configuration DSL to JSON Schema

## What is it ?

This is is a Rust (edition 2018) library that is intended to be used as a translation mechanism between the [configuration DSL](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md) and the `JSON + UI Schema` world.  
The is not production ready yet, however it supports some real-life cases already.  
The plan is to have this library be available to be used from Rust code but also as a Node module to be used from Node and in-browser code, through WASM.

## How to use it ?

The main entry point to the library is the `Generator` type and its `generate` type.
It consumes [`serde_yaml`](https://crates.io/crates/serde_yaml) values and outputs a tuple of [`serde_json`](https://crates.io/crates/serde_json) values.

Example of use:
```rust
use serde_yaml;
use serde_json;
use balena_configuration_dsl::Generator;

let input_schema : serde_yaml::Value = serde_yaml::from_str(
    include_str!("input-schema.yml")).
    unwrap();

let (json_schema, ui_object) = Generator::with(input_schema)?.generate();
```

For examples of input yaml DSL schemas please see the ones used in tests in [`tests/data`](./tests/data) directory.


## What is supported ?

All basic types from the [main DSL specification](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md) are supported.
The support is still pretty shallow, i.e. while the feature may be supported its edge cases are probably not supported.
Please find a detailed list in the [SPEC_SUPPORT](./SPEC_SUPPORT.md) document.

## Contributing

Please see [implementors' notes](./NOTES.md) for the current scratchpad and TODOs.  
This will be moved into Github issues.

