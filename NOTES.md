


Guidelines: 
* deserialization bit should be as simple as possible, not much logic there
* serialization should have all the computation, the typesystem should be as small as possible though

TODOs:
* low level code quality: 
  * make serde use consistent, right now there is a mix of attributes, `with` functions and `impl (De)serialize`
  * visibility: remove pub access where possible
  * lifetimes: remove unnecessary clones
  * most of the above are marked with `todo` or `fixme` in the code
* after adding support for basic features:do a sweep through the spec and add tests for edge cases etc
* test edge cases - generate with property based tests
* add fuzzer (afl?) to see if there are no crashes on any input
* when used in other projects - create a `crater-like` test suite where we take the latest raleases stable version of the project that uses the library and then run its test suite with previous stable version of this library and with the new one, pre-release


Compiler phases:
* phases (traits + eventually macros, see `Validate` trait): - effectively compilation-level state machine
  * deserialize from yaml - in general this creates values, other phases have references to them
  * normalize (e.g. copy title into value if needed)
  * trim (e.g. strip empty objects)
  * validate
  * serialize


Spec notes:
* password field - should it have associated `UIObject` widget ?

