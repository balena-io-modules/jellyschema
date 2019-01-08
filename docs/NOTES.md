These are implementation notes - a current scratchpad for the maintainers.

Temen notes:
* not immediately obvious that `Identifier` can be multiple identifiers
Spec notes:
* sometimes JSON is represented as yaml in the spec - I found it harder to scan the spec for the relevant examples this way
* "The value of this keyword MUST be either a string or an array. If it is an array, elements of the array MUST be strings and MUST be unique."
   String values MUST be one of the seven primitive types"
   - on the first reading not clear the difference between yaml types and dsl types
* 'schema' is a very overloaded term - it may be hard to grasp what is meant byt 'schema' in different parts of the spec
