#!/usr/bin/env bash

set -e
set -o pipefail

RUSTUP_COMMAND="curl https://sh.rustup.rs -sSf | sh -s -- -y"
if [ ! -z "${CI}" ]; then
    default_compiler=`cat rust-toolchain`
    echo "Setting the default compiler to ${default_compiler}"
    RUSTUP_COMMAND="$RUSTUP_COMMAND --default-toolchain ${default_compiler}"
fi
echo "Installing Rust toolchain..."
eval "${RUSTUP_COMMAND}"

source "${HOME}/.cargo/env"
rustup component add clippy
rustup component add rustfmt

echo "Updating Rust toolchain..."
(test -x "${HOME}/.cargo/bin/cargo-install-update" || cargo install cargo-update)
cargo install-update -a

echo "Installing NVM & NodeJS..."
curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | bash
source "${HOME}/.nvm/nvm.sh"
nvm install
nvm use
echo "NodeJS version $(node --version)"

echo "Installing wasm-pack..."
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f

echo "Installing yaml linter..."
pip install --user -r requirements.txt

echo "Installing JSONSchema linter..."
npm install -g ajv-cli
