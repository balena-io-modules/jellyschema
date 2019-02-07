# Jelly Schema

[![Build Status](https://travis-ci.org/balena-io-modules/jellyschema.svg?branch=master)](https://travis-ci.org/balena-io-modules/jellyschema)
[![Current Release](https://img.shields.io/github/tag/balena-io-modules/jellyschema.svg?style=flat-square)](https://github.com/balena-io-modules/jellyschema/tags)
[![License](https://img.shields.io/github/license/balena-io-modules/jellyschema.svg?style=flat-square)](https://github.com/balena-io-modules/jellyschema/blob/master/LICENSE)
[![Issues](https://img.shields.io/github/issues/balena-io-modules/jellyschema.svg?style=flat-square)](https://github.com/balena-io-modules/jellyschema/issues)

<div align="center">
  <sub>an open source :satellite: project by <a href="https://www.balena.io">balena.io</a></sub>
</div>

A configuration DSL.

Provides facilities to:

* transform configuration DSL into the JSON Schema & UI Object Schema with custom extensions
* parse configuration DSL

Current crate status is _experimental_. It's because the API is evolving, we're not fully using
it yet and we will probably move the transformation functionality to another crate in the future.
This crate will become a simple configuration DSL parser and nothing else.

## Goal

`jellyschema` crate is one small piece of the [balena.io] configuration project. This project has
no public / open specification yet, but we're working on it and it will be public once finished.

## Supported platforms

This library is written in the Rust language and can be used:

* directly, as a [Rust crate]
* as an isomorphic [NPM package] (NodeJS & browser)

## Documentation

* [API documentation]
* [Changelog]
* [Maintainer documentation]

## Usage

### Rust

Add as a dependency to your `Cargo.toml`:

```
[dependencies]
jellyschema = "0"
```

Evaluate simple JSON:

```rust
let input_schema: serde_yaml::Value = serde_yaml::from_str(
    include_str!("configuration.yml")).
    unwrap();

let (json_schema, ui_object) = Generator::with(input_schema)?.generate();
```

### Javascript

Install via npm

```
npm install --save jellyschema
```

Generate simple JSON Schema & UI Object Schema:

```js
const jellyschema = require('jellyschema');

const initialValue = `
title: demo
version: 1
properties:
  - network:
      title: Network
      properties:
        - ssid:
            title: Network SSID
            type: string
            minLength: 1
            maxLength: 32
        - passphrase:
            title: Network Key
            type: password
            minLength: 8
`;

var schema = new jels.JellySchema(initialValue);
const result = schema.jsonAndUiSchema();
console.log(JSON.stringify(result, null, 2));

console.log(schema.validate({network: { ssid: 'foo', passphrase: 123 }}));
console.log(schema.errors());
```

An example of using this module in nodeJS is available in the `examples/node` folder:

```bash
cd examples/node
npm install
npm start
```

An example of using this module in the browser is available in the `examples/browser` folder:

```bash
cd examples/browser
npm install
npm start
```

Open `localhost:8080` in your browser.

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
[Maintainer documentation]: ./docs/MAINTAINER.md
