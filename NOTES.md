Ideas for code shape:

* phases (traits + eventually macros, see `Validate` trait): - effectively compilation-level state machine
  * deserialize from yaml - in general this creates values, other phases have references to them
  * normalize (e.g. copy title into value if needed)
  * validate
  * trim (e.g. strip empty objects)
  * serialize
* the serialization into final json should be as trivial as possible - use just serde derives when possible
  
TODOs:
* low level code quality: remove pub access, remove unnecessary clones
* after adding support for basic features:do a sweep through the spec and add tests for edge cases etc