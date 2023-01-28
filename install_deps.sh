#!/bin/bash

apt update
apt install -y python3-pip
pip install -U pip
pip install pre-commit

# apt install -y shfmt

rustup toolchain install nightly
rustup default nightly
rustup component add rustfmt clippy
