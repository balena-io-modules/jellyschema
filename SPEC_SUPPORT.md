# What is supported ?

All basic types from the [main DSL specification](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md) are supported.
The support is still pretty shallow, i.e. while the feature may be supported its edge cases are probably not supported.
Please find a detailed list below - presented in the order they appear in the spec.

* [X] means the feature is supported, with the caveat as per above - it works with simple happy path examples, does not mean just yet that it will work for all possible examples
* [ ] means there is no support

[Meta schema](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#meta-schema)
* [X] `version`
* [X] `title`

[Validation keywords](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#validation-keywords)

* [X] `type`
* [X] `optional type`
* [X] `password`
* [ ] [dates and times](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#dates-and-times)
* [X] `hostname` but with no hostname validation
* [ ] [ip addresses](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#ip-addresses)
* [ ] `uri`
* [ ] `data`

* [X] [enum](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#enum) - only for string instances, should be trivial to add for other types
* [X] [const](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#const)- only for string instances, should be trivial to add for other types

[Validation keywords for numeric instances](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#validation-keywords-for-numeric-instances-number-and-integer)
* [X] `multipleOf`
* [X] `maximum`
* [X] `exclusiveMaximum`
* [X] `minimum`
* [X] `exclusiveMinimum`

[Validation keywords for strings](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#validation-keywords-for-strings)
* [X] `maxLength`
* [X] `minLength`
* [X] `pattern` - validated as Rust regex

[Validation keywords for arrays](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#validation-keywords-for-arrays)
* [X] `items` - both flavours (one item and consecutive items)
* [X] `additionalItems`
* [X] `maxItems`
* [X] `minItems`
* [X] `uniqueItems` - both flavours (flag or an array)

[Validation keywords for objects](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#validation-keywords-for-objects)
* [X] `properties` - 'recursive' schemas are supported

[Keywords for applying subschemas conditionally](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#keywords-for-applying-subschemas-conditionally)
* [ ] `when` - very rudimentary support, only single-identifier boolean conditions
* [ ] `merge`
* [ ] `overwrite`

[Templating](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#templating)
* None of templating section is supported yet - we need to integrate already existing [temen](https://github.com/balena-io-modules/balena-temen) library.

[Schema annotations](https://github.com/balena-io/balena/blob/832f5551127dd8e1e82fa082bea97fc4db81c3ce/specs/configuration-dsl.md#schema-annotations)
* [X] `title`
* [X] `description`
* [X] `help`
* [X] `warning`
* [X] `default`, but with no support for `eval` yet
* [ ] `readOnly`
* [ ] `writeOnly`
* [ ] `hidden`
* [ ] `collapsed`
* [ ] `collapsible`
* [ ] `orderable`
* [ ] `addable`
* [ ] `removable`
* [ ] `placeholder`


