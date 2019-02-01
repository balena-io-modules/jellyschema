#!/usr/bin/env bash
set -e
set -o pipefail
HERE="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PARENT="$HERE/../"

(test -x "${HOME}/.cargo/bin/cargo-fuzz" || cargo install cargo-fuzz)

# it is important for `cargo fuzz` to be run from the project root
cd "$PARENT"

# run `any_input` target using `seeds` as a start point and put new corpus state into `corpus`
cargo fuzz run any_input "$HERE/corpus" "$HERE/seeds" -j 6 --all-features
