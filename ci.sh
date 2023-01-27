#!/bin/bash

install_toolchain() {
	apt update
	apt install -y \
		build-essential \
		curl \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	echo "export PATH=\"$HOME/.cargo/bin:$PATH\"" >>~/.bashrc
	source $HOME/.cargo/env
	# please run `source ~/.bashrc` in command line.
}

update_toolchain() {
	rustup toolchain install nightly
	rustup default nightly
	rustup component add rustfmt clippy
	rustup update
	cargo update --verbose
}

ci() {
	if ! command -v cargo &>/dev/null; then
		echo "command not found"
		setup
	fi

	update_toolchain

	cargo package --list --allow-dirty

	# lint
	cargo clippy

	# format
	cargo fmt
	./../ci.sh

	# test
	cargo test -q --release

	cargo publish --dry-run --allow-dirty

	$@
}

ci $@
