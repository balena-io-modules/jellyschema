This are implementation notes - a current scratchpad for the maintainers.

Guidelines: 
* deserialization bit should be as simple as possible, not much logic there
* serialization should have all the computation, the typesystem should be as small as possible though

Temen notes:
* not immediately obvious that `Identifier` can be multiple identifiers
Spec notes:
* sometimes JSON is represented as yaml in the spec - I found it harder to scan the spec for the relevant examples this way
* "The value of this keyword MUST be either a string or an array. If it is an array, elements of the array MUST be strings and MUST be unique." 
   String values MUST be one of the seven primitive types"
   - on the first reading not clear the difference between yaml types and dsl types
* `max/min` vs `maximum/minimum` in names of the validations
* is not specifying `items` in an array type okay ?
* 'schema' is a very overloaded term - it may be hard to grasp what is meant byt 'schema' in different parts of the spec

Roadmap:
* Create a playground application that would allow other people to play with the DSL
* when the spec is merged - mass-change links in the `SPEC_SUPPORT.md`
