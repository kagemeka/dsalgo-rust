#!/bin/bash

install_deps() {
	apt update
	apt install -y python3-pip
	pip install -U pip
	pip install pre-commit

	# apt install -y shfmt

	rustup toolchain install nightly
	rustup default nightly
	rustup component add rustfmt clippy
}

ci() {
	install_deps
	rustup update
	cargo update

	cargo package --list --allow-dirty

	cargo clippy

	cargo fmt --all
	# shfmt -w **/*.sh
	pre-commit run --all-files

	cargo test -q --release

	cargo publish --dry-run --allow-dirty

	$@
}

ci $@
