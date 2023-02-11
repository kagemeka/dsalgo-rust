#!/bin/bash

rustup update
cargo update
# cargo package --list --allow-dirty
cargo fmt -v --all
cargo clippy
pre-commit run --all-files
cargo test -q
# cargo test -q --release
# cargo publish --dry-run --allow-dirty
