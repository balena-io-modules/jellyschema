# Change Log

All notable changes to this project will be documented in this file
automatically by Versionist. DO NOT EDIT THIS FILE MANUALLY!
This project adheres to [Semantic Versioning](http://semver.org/).

# v0.11.10
## (2019-03-21)

* Update to newest version of webpack [Cyryl Płotnicki]

# v0.11.9
## (2019-03-18)

* Set project type to rust-public-crate-wasm [Giovanni Garufi]

# v0.11.8
## (2019-03-14)

* Introduce fill_default_values [Robert Vojta]

# v0.11.7
## (2019-02-26)

* Rename the demo examples directory [Cyryl Płotnicki]

# v0.11.6
## (2019-02-26)

* Add support for `collapsed` and `collapsible` [Cyryl Płotnicki]

# v0.11.5
## (2019-02-15)

* Emit description in JSON schema, not UI schema [Robert Vojta]

# v0.11.4
## (2019-02-12)

* Report serde errors [Robert Vojta]

# v0.11.3
## (2019-02-11)

* Emit UI Schema for array.items [Robert Vojta]

# v0.11.2
## (2019-02-08)

* Ensure the correct Rust version when running locally and on CI. [Cyryl Płotnicki]

# v0.11.1
## (2019-02-08)

* Cleanup after reconfix functionality merge [Robert Vojta]

# v0.11.0
## (2019-02-07)

* Rename & merge reconfix functionality [Robert Vojta]

# v0.10.5
## (2019-02-05)

* `version` keyword available and passed through on every schema level. [Cyryl Płotnicki]

# v0.10.4
## (2019-02-05)

* Generate wasm bindings conditionally [Robert Vojta]

# v0.10.3
## (2019-02-04)

* Add unhappy-path tests. [Cyryl Płotnicki]
* Rename uiobject to uischema in test file names. [Cyryl Płotnicki]

# v0.10.2
## (2019-02-01)

* Fix running fuzzer from scratch. [Cyryl Płotnicki]

# v0.10.1
## (2019-02-01)

* Add basic fuzzer configuration. [Cyryl Płotnicki]

# v0.10.0
## (2019-02-01)

* Rename JS file in the NPM package [Robert Vojta]

# v0.9.2
## (2019-01-30)

* Emit `ui:order` in addition to `$$order` [Cyryl Płotnicki]

# v0.9.1
## (2019-01-30)

* Emit both `writeOnly` and `ui:widget`:`password` for `writeOnly` [Cyryl Płotnicki]
* Emit `ui:widget`:`password` for passwords, in addition to `writeOnly` [Cyryl Płotnicki]
* Emit `readOnly` on the JSONSChema level as well. [Cyryl Płotnicki]

# v0.9.0
## (2019-01-30)

* Remove support for `when` [Cyryl Płotnicki]

# v0.8.0
## (2019-01-29)

* Remove mapping passthrough. [Cyryl Płotnicki]

# v0.7.6
## (2019-01-29)

* Add readOnly annotation [Cyryl Płotnicki]
* Add support for placeholder. [Cyryl Płotnicki]
* Add support for `hidden` keyword. [Cyryl Płotnicki]

# v0.7.5
## (2019-01-29)

* Add support for array annotations. [Cyryl Płotnicki]

# v0.7.4
## (2019-01-28)

* Add support for pass-through of `additionalProperties` [Cyryl Płotnicki]
* Add support for dynamic objects. [Cyryl Płotnicki]

# v0.7.3
## (2019-01-23)

* Add support for *address types [Cyryl Płotnicki]

# v0.7.2
## (2019-01-22)

* Allow lenght bounds and patterns to coexist on strings [Cyryl Płotnicki]
* Add more tests for const, enum and pattern [Cyryl Płotnicki]
* Add enum/const support for booleans [Cyryl Płotnicki]
* Add support for all string object bounds for all string-based types [Cyryl Płotnicki]

# v0.7.1
## (2019-01-21)

* Move to Rust stable [Cyryl Płotnicki]

# v0.7.0
## (2019-01-18)

* Remove support for compund types completely [Cyryl Płotnicki]

# v0.6.13
## (2019-01-18)

* Add `stringlist` type [Cyryl Płotnicki]

# v0.6.12
## (2019-01-17)

* Add support for annotations on the root level [Cyryl Płotnicki]

# v0.6.11
## (2019-01-17)

* Fix port type to be integer on JSONSchema side [Cyryl Płotnicki]

# v0.6.10
## (2019-01-16)

* Add support for `file` type [Cyryl Płotnicki]
* Add widget support for single binary type case [Cyryl Płotnicki]

# v0.6.9
## (2019-01-15)

* Fix problems with nested UISchema [Cyryl Płotnicki]

# v0.6.8
## (2019-01-10)

* Add support for `formula` keyword on the schema level [Cyryl Płotnicki]

# v0.6.7
## (2019-01-10)

* Support default keyword for all types generically [Cyryl Płotnicki]
* Add default value support for Number and Port [Cyryl Płotnicki]
* Add support for default values on Integers [Cyryl Płotnicki]

# v0.6.6
## (2019-01-07)

* Remove stale documentation [Cyryl Płotnicki]

# v0.6.5
## (2019-01-07)

* Fix WASM build. [Cyryl Płotnicki]

# v0.6.4
## (2018-12-20)

* Unknown types are treated as `text` by default [Cyryl Płotnicki]

# v0.6.3
## (2018-12-20)

* Disable travis cache [Cyryl Płotnicki]
* Add support for `port` type [Cyryl Płotnicki]
* Add definitions for types that are passthrough to JSONSchema types. [Cyryl Płotnicki]
* Add `number` type [Cyryl Płotnicki]
* Add support for `text` type [Cyryl Płotnicki]

# v0.6.2
## (2018-12-18)

* Add support for enum values on integers [Cyryl Płotnicki]
* Extract generic enumeration deserialization [Cyryl Płotnicki]

# v0.6.1
## (2018-12-18)

* Update temen dependency [Cyryl Płotnicki]

# v0.6.0
## (2018-12-17)

* Make sure that we can only use `1` as a version number [Cyryl Płotnicki]
* Make the top level schema version optional [Cyryl Płotnicki]

# v0.5.0
## (2018-12-17)

* New multiple `items` semantics [Cyryl Płotnicki]

# v0.4.0
## (2018-12-17)

* Remove support for `additionalItems` keyword [Cyryl Płotnicki]

# v0.3.0
## (2018-12-17)

* Switch to beta toolchain [Cyryl Płotnicki]
* Normalize min/max property names according to the new spec. [Cyryl Płotnicki]

# v0.2.10
## (2018-12-06)

* Rust toolchain stable [Robert Vojta]

# v0.2.9
## (2018-12-06)

* Do not require jest to be installed globally [Cyryl Płotnicki]

# v0.2.8
## (2018-12-06)

* Remove -preview suffix for clippy, rustfmt [Robert Vojta]
* Add changelog link to readme [Robert Vojta]
* Replace wasm32 triplet with target_arch [Robert Vojta]

# v0.2.7
## (2018-11-28)

* Add missing description [Robert Vojta]

# v0.2.6
## (2018-11-28)

* Initial release [Robert Vojta]

# v0.2.5
## (2018-11-26)

* Add interactive example of usage in the browser [Cyryl Płotnicki]
* Add headlesss browsers to CI environment [Cyryl Płotnicki]
* Expose UI generation function as WASM function [Cyryl Płotnicki]
* Port wasm build over from temen [Cyryl Płotnicki]
* Add node.js as dependency [Cyryl Płotnicki]

# v0.2.4
## (2018-11-23)

* Add balena-temen as dependency [Cyryl Płotnicki]

# v0.2.3
## (2018-11-15)

* Remove explicit travis notifications [Cyryl Płotnicki]

# v0.2.2
## (2018-11-15)

* Update dependencies [Cyryl Płotnicki]

# v0.2.1
## (2018-11-14)

* Remove TODOs from the NOTES.md, transformed into github issues [Cyryl Płotnicki]

# v0.2.0
## (2018-11-14)

* Set change type for versionist [Cyryl Płotnicki]

## v0.1.4 - 2018-10-17

* Add resin-cli demo test [Robert Vojta]

## v0.1.3 - 2018-10-17

* Add basic README with a TODO list [Cyryl Płotnicki]

## v0.1.2 - 2018-10-17

* Update email address [Robert Vojta]

## v0.1.1 - 2018-10-16

* Add rust version specification for the tooling [Cyryl Płotnicki]

## v0.1.0 - 2018-10-16

* Add versionbot support [Giovanni Garufi]
* Add library skeleton [Robert Vojta]
