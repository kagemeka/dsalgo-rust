# DsAlgo Rust

Data Structures and Algorithms written in Rust.

[![Crates.io][crates-badge]][crates-url]
[![Github pages][gh-pages-badge]][gh-pages-url]
[![MIT licensed][mit-badge]][mit-url]
[![CI][actions-badge]][actions-url]
[![pre-commit][pre-commit-badge]][pre-commit-url]

[crates-badge]: https://img.shields.io/crates/v/dsalgo.svg
[crates-url]: https://crates.io/crates/dsalgo
[gh-pages-badge]: https://github.com/kagemeka/dsalgo-rust/actions/workflows/pages/pages-build-deployment/badge.svg
[gh-pages-url]: https://kagemeka.github.io/dsalgo-rust
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/kagemeka/dsalgo-rust/blob/main/LICENSE
[actions-badge]: https://github.com/kagemeka/dsalgo-rust/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/kagemeka/dsalgo-rust/actions/workflows/rust.yml
[pre-commit-badge]: https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white
[pre-commit-url]: https://github.com/pre-commit/pre-commit

## Installation

upcoming.
now please see crates.io

## Development

### Prerequisites

- docker installed.

### create docker container and enter into it

we recommend you to use VSCode and the extension `Dev Containers`.
in that case, after the command below completed, you can select `Docker: Focus on Container Views` from command palette.

```sh
docker compose up -d
```

### VSCode Extensions

if you use VSCode, it gonna be fine to install these extensions.

- rust-analyzer
- Even Better TOML

### Setup & CI

in addition to base rust docker image, you should install some dependencies.
don't worry, just do it.

```sh
./ci.sh
```

### See document

- <https://doc.rust-lang.org/cargo/commands/cargo-doc.html>

```sh
cargo doc --open
```

### Clean targets

- <https://doc.rust-lang.org/cargo/commands/cargo-clean.html>
- use when something is wrong.

```sh
cargo clean
```

### Check easy compilation errors

- <https://doc.rust-lang.org/cargo/commands/cargo-check.html>

```sh
cargo check
```
