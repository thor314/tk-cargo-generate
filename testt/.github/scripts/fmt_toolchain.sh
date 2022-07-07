#!/usr/bin/env bash
set -e # exit immediately if command ends with non-zero exit status.

# sudo apt install -y libssl-dev clang libclang-dev
# rustup update stable
# rustup update nightly
rustup default nightly
rustup component add rustfmt
# rustup target add wasm32-unknown-unknown --toolchain nightly
