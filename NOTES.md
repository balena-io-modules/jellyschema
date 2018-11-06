


Guidelines: 
* deserialization bit should be as simple as possible, not much logic there
* serialization should have all the computation, the typesystem should be as small as possible though


Compiler phases:
* phases (traits + eventually macros, see `Validate` trait): - effectively compilation-level state machine
  * deserialize from yaml - in general this creates values, other phases have references to them
  * normalize (e.g. copy title into value if needed)
  * trim (e.g. strip empty objects)
  * validate
  * serialize

TODOs:
* support properties on every level - recursive schema
* low level code quality: 
  * make serde use consistent, right now there is a mix of attributes, `with` functions and `impl (De)serialize`
  * visibility: remove pub access where possible
  * lifetimes: remove unnecessary clones
  * most of the above are marked with `todo` or `fixme` in the code
* after adding support for basic features:do a sweep through the spec and add tests for edge cases etc

Spec notes:
* password field - should it have associated `UIObject` widget

