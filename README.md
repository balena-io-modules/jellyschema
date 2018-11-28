# balena cdsl

[![Build Status](https://travis-ci.org/balena-io-modules/balena-cdsl.svg?branch=master)](https://travis-ci.org/balena-io-modules/balena-cdsl)
[![Current Release](https://img.shields.io/github/tag/balena-io-modules/balena-cdsl.svg?style=flat-square)](https://github.com/balena-io-modules/balena-cdsl/tags)
[![License](https://img.shields.io/github/license/balena-io-modules/balena-cdsl.svg?style=flat-square)](https://github.com/balena-io-modules/balena-cdsl/blob/master/LICENSE)
[![Issues](https://img.shields.io/github/issues/balena-io-modules/balena-cdsl.svg?style=flat-square)](https://github.com/balena-io-modules/balena-cdsl/issues)

<div align="center">
  <sub>an open source :satellite: project by <a href="https://www.balena.io">balena.io</a></sub>
</div>

A configuration DSL.

Provides facilities to:

* transform configuration DSL into the JSON Schema & UI Object Schema with custom extensions
* parse configuration DSL

## Goal

`balena-cdsl` crate is one small piece of the [balena.io] configuration project. This project has
no public / open specification yet, but we're working on it and it will be public once finished.

## Supported platforms

This library is written in the Rust language and can be used:

* directly, as a [Rust crate]
* as an isomorphic [NPM package] (NodeJS & browser)

## Documentation

* [API documentation]

## Usage

### Rust

Add as a dependency to your `Cargo.toml`:

```
[dependencies]
balena-cdsl = "0.2"
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
npm install --save balena-cdsl
```

Generate simple JSON Schema & UI Object Schema:

```js
const cdsl = require('balena-cdsl');

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

console.log(cdsl.generate_ui(initialValue));
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

`balena-cdsl` is open source software, and may be redistributed under the terms specified in
the [license].

[balena.io]: https://www.balena.io/
[contact us]: https://forums.balena.io/
[raise an issue]: https://github.com/balena-io-modules/balena-cdsl/issues/new
[API documentation]: https://docs.rs/balena-cdsl/latest/balena_cdsl/
[license]: https://github.com/balena-io-modules/balena-cdsl/blob/master/LICENSE
[Rust crate]: https://crates.io/crates/balena-cdsl
[NPM package]: https://www.npmjs.com/package/balena-cdsl
