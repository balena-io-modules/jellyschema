#!/usr/bin/env bash

set -e

HERE="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

source "${HOME}/.cargo/env"
source "${HOME}/.nvm/nvm.sh"
nvm use

echo "Setting rustup override for this project"
rustup override set $(cat rust-toolchain)

echo "Authenticating to crates.io..."
cargo login "${CARGO_API_TOKEN}"
echo "Publishing Rust crate..."
cargo publish

echo "Authenticating to npmjs.org registry..."
echo "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" > ~/.npmrc

"$HERE/build-wasm.sh"

echo "Publishing NPM package..."
npm publish --access public target/npm/pkg
