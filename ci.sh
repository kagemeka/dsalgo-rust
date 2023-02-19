#!/bin/bash

rustup update
cargo update
cargo fmt
cargo clippy --tests --all-features
pre-commit run --all-files
cargo test #--release
