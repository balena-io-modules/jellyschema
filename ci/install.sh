#!/usr/bin/env bash

set -e
set -o pipefail

rustup default `cat rust-toolchain`
rustup component add clippy-preview
rustup component add rustfmt-preview
pip install --user -r requirements.txt
