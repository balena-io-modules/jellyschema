# Jelly Schema

[![Current Release](https://img.shields.io/github/tag/balena-io-modules/jellyschema.svg?style=flat-square)](https://github.com/balena-io-modules/jellyschema/tags)
[![License](https://img.shields.io/github/license/balena-io-modules/jellyschema.svg?style=flat-square)](https://github.com/balena-io-modules/jellyschema/blob/master/LICENSE)
[![Issues](https://img.shields.io/github/issues/balena-io-modules/jellyschema.svg?style=flat-square)](https://github.com/balena-io-modules/jellyschema/issues)

<div align="center">
  <sub>an open source :satellite: project by <a href="https://www.balena.io">balena.io</a></sub>
</div>

Provides facilities to:

* parse Jelly Schema
* validate JSON data against Jelly Schema.

Current crate status is **experimental**.

## Supported platforms

This library is written in the Rust language and can be used:

* directly, as a [Rust crate]
* as an isomorphic [NPM package] (NodeJS & browser)

**NOTE:** Not all features are available in the NPM package. Check the
[`wasm.rs`](./src/wasm.rs) module for more details. 

## Documentation

* [Specification]
* [How to add a new data type]
* [API documentation]
* [Examples]
* [Changelog]

## Support

If you're having any problem, please [raise an issue] on GitHub or [contact us], and the [balena.io] team
will be happy to help.

## License

`jellyschema` is open source software, and may be redistributed under the terms specified in
the [license].

[balena.io]: https://www.balena.io/
[contact us]: https://forums.balena.io/
[raise an issue]: https://github.com/balena-io-modules/jellyschema/issues/new
[API documentation]: https://docs.rs/jellyschema/latest/jellyschema/
[license]: ./LICENSE
[Rust crate]: https://crates.io/crates/jellyschema
[NPM package]: https://www.npmjs.com/package/jellyschema
[Changelog]: ./CHANGELOG.md
[Specification]: ./docs/specification.md
[How to add a new data type]: ./docs/add-data-type.md
[Examples]: ./examples/
