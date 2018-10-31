Ideas for code shape:

* phases (traits + eventually macros, see `Validate` trait): - effectively compilation-level state machine
  * deserialize from yaml - in general this creates values, other phases have references to them
  * normalize (e.g. copy title into value if needed)
  * trim (e.g. strip empty objects)
  * validate
  * serialize

* the serialization into final json should be as trivial as possible - use just serde derives when possible
  
TODOs:
* support properties on every level - recursive schema
* low level code quality: 
  * make serde use consistent, right now there is a mix of attributes, `with` functions and `impl (De)serialize`
  * visibility: remove pub access where possible
  * lifetimes: remove unnecessary clones
  * most of the above are marked with `todo` or `fixme` in the code
* after adding support for basic features:do a sweep through the spec and add tests for edge cases etc