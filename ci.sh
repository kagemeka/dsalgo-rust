#!/bin/bash

rustup update
cargo update

cargo package --list --allow-dirty

cargo clippy

cargo fmt --all
# shfmt -w **/*.sh
pre-commit run --all-files

cargo test -q --release

cargo publish --dry-run --allow-dirty
