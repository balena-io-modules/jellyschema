


Guidelines: 
* deserialization bit should be as simple as possible, not much logic there
* serialization should have all the computation, the typesystem should be as small as possible though

TODOs:
* low level code quality: 
  * make serde use consistent, right now there is a mix of attributes, `with` functions and `impl (De)serialize`
  * visibility: remove pub access where possible
  * lifetimes: remove unnecessary clones
  * most of the above are marked with `todo` or `fixme` in the code
  * validate `min/max` etc values cannot be negative, use `u64` instead of `i64`
  * fix debug formatting of serde values is errors - very unreadable right now
  * have a type for `non-empty vec` and use it pretty much everywhere where `Option<Vec>` is used now
   
* docs
* after adding support for basic features:do a sweep through the spec and add tests for edge cases etc
* test edge cases - generate with property based tests
* add fuzzer (afl?) to see if there are no crashes on any input
* when used in other projects - create a `crater-like` test suite where we take the latest raleases stable version of the project that uses the library and then run its test suite with previous stable version of this library and with the new one, pre-release
* use `valico` to validate the json schemas in tests
* use `repo.yaml`

Compiler phases:
* phases (traits + eventually macros, see `Validate` trait): - effectively compilation-level state machine
  * deserialize from yaml - in general this creates values, other phases have references to them
  * normalize (e.g. copy title into value if needed)
  * trim (e.g. strip empty objects)
  * validate
  * serialize


Spec notes:
* password field - should it have associated `UIObject` widget ?
* sometimes JSON is represented as yaml in the spec - I found it harder to scan the spec for the relevant examples this way
* "The value of this keyword MUST be either a string or an array. If it is an array, elements of the array MUST be strings and MUST be unique." 
   String values MUST be one of the seven primitive types"
   - on the first reading not clear the difference between yaml types and dsl types
* `max/min` vs `maximum/minimum` in names of the validations

Roadmap:
* Create a playground application that would allow other people to play with the DSL
