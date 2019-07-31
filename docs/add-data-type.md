# How to add a new data type

## Pick a name

Data type names are lower cased and use hyphens. Examples:

* `port`
* `date-time`
* etc.

In this document, we will add `even-unprivileged-port` data type.
What does it mean? Even number in the 1024-65535 range with generator.

## Simple data type module

All data types are stored in the [data_types](.../src/data_types)
folder.

### Module folder

Create `src/data_types/even_unprivileged_port` folder.

### Rust module

Create `even_unprivileged_port/mod.rs` module file with
the following content:

```rust
data_type!("even-unprivileged-port");
```

### Data type schema

Every data type MUST provide a schema. Create `even_unprivileged_port/schema.yaml`
file with the following content:

```yaml
type: integer
min: 1024
max: 65535
```

This schema says that the data type must be a number within the 1024-65535 range.

### Include data type in the Jelly Schema

Open `src/data_types/mod.rs` file and add your data type module ...

```rust
mod even_unprivileged_port;
```

... and include it in the data types list ...

```rust
data_types!(
  ...,
  even_unprivileged_port,
  ...
);
```

... and keep it sorted alphabetically.

We can use the new `even-unprivileged-port` data type in this way:

```yaml
type: even-unprivileged-port
```

### Optional validation tests

It's optional step, but it's recommended to add the validation tests. You can
do it by creating `even_unprivileged_port/validation.yaml` file with the
following content:

```yaml
schema:
  type: even-unprivileged-port
tests:
  - valid: true
    description: Must be valid if it's in the 1024 - 65535 range
    data: 1028
  - valid: false
    description: Must be invalid if it's lower than 1024
    data: 1023
  - valid: false
    description: Must be invalid if it's greater than 65535
    data: 65536
```

* `schema` keyword contains the schema for data validation tests
* `tests` is an array of tests
* `valid` says if the provided `data` should be valid or not
* `description` is used in panics (test output)

Run `cargo test` and you should see the following output for this
data type:

```
test dt::src::data_types::even_unprivileged_port::validation ... ok
```

## Custom data type validator

We can't check if the number is even or odd with the schema only and
we have to add custom validator. It's a simple function with the following
signature:

```rust
fn validate(data: &Value, ctx: &WalkContext) -> ValidationState;
```

Update your `even_unprivileged_port/mod.rs` module with the following
content:

```rust
use serde_json::Value;

use crate::validators::{ValidationState, WalkContextExt};
use crate::WalkContext;

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    match data.as_u64() {
        // Good, even number, return empty ValidationState (= no errors)
        Some(number) if number % 2 == 0 => ValidationState::new(),
        // Bad, number, but it's not even, return ValidationState with error
        Some(_) => ctx.validation_error("type", "expected even number").into(),
        // It's not a number, return empty ValidationState. The reason
        // for this is that schema says `type: integer` and this keyword
        // is validated as well. In other words,
        //
        // The rule is - if type differs, ignore it and return empty
        // validation state.
        None => ValidationState::new(),
    }
}

// Here we have to pass not just type name, but also a validation function
data_type!("even-unprivileged-port", validator: validate);
```

And that's it. You have a custom data type validator. Data type is validated against
the data type schema and then your custom validator is executed as well.

We can update `even_unprivileged_port/validation.yaml` test cases:

```yaml
schema:
  type: even-unprivileged-port
tests:
  - valid: true
    description: Must be valid if it's in the 1024 - 65535 range
    data: 1028
  # New one to check if the 'even number' validation works
  - valid: false
    description: Must be invalid if it's in the 1024 - 65535 range, but is not even
    data: 2021
  - valid: false
    description: Must be invalid if it's lower than 1024
    data: 1023
  - valid: false
    description: Must be invalid if it's greater than 65535
    data: 65536
```

The `cargo run` says it works:

```
test dt::src::data_types::even_unprivileged_port::validation ... ok
```

## Custom data type value generator

It's simple as adding the custom data validator. Update your
`even_unprivileged_port/mod.rs` module with the following content:

```rust
use rand::Rng;
use serde_json::{json, Value};

use crate::generator;
use crate::validators::{ValidationState, WalkContextExt};
use crate::{Schema, WalkContext};

fn validate(data: &Value, ctx: &WalkContext) -> ValidationState {
    match data.as_u64() {
        // Good, even number, return empty ValidationState (= no errors)
        Some(number) if number % 2 == 0 => ValidationState::new(),
        // Bad, number, but it's not even, return ValidationState with error
        Some(_) => ctx.validation_error("type", "expected even number").into(),
        // It's not a number, return empty ValidationState. The reason for this is that
        // schema says `type: integer` and this keyword is validated as well. In other words,
        // if type differs, we can ignore it
        None => ValidationState::new(),
    }
}

fn generate(_schema: &Schema) -> generator::Result<Value> {
    let mut rng = rand::thread_rng();

    let result = loop {
        // Generate random number within the 1024-65536 range
        // 1024 - included, 65536 - excluded
        let number = rng.gen_range(1024, 65536);

        // If it's even, return it
        if number % 2 == 0 {
            break number;
        }
    };

    Ok(json!(result))
}

// Here we have to pass not just type name, but also validator and
// generator functions
data_type!("even-unprivileged-port", validator: validate, generator: generate);
```
