#!/usr/bin/env bash
set -e # exit immediately if command ends with non-zero exit status.

# necessary for rustfmt full features
# rustup default nightly
# rustup component add rustfmt

# ✨check test..let someone else fmt✨
cargo check
cargo test --all-features --verbose
# cargo fmt --all -- --check