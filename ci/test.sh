#!/usr/bin/env bash

set -e
set -o pipefail

TESTS_DIRECTORY=tests

find "$TESTS_DIRECTORY" -iname *.yml -exec yamllint {} +

find "$TESTS_DIRECTORY" -type f -iname "output-json-schema.json" -print0 | while IFS= read -r -d $'\0' file; do
    ajv compile -s "$file" --format=full
done

if [ ! "$CI" == "true" ]; then
    # When running locally, we have to clean the project, otherwise clippy
    # won't do nothing if the project was already compiled
    cargo clean
fi
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo fmt -- --check

if [ ! "$CI" == "true" ]; then
    # Allow uncommitted changes when running locally
    CARGO_PACKAGE_FLAGS="--allow-dirty"
fi

cargo package ${CARGO_PACKAGE_FLAGS}
