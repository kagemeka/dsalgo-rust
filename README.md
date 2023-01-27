# DsAlgo Rust

Data Structures and Algorithms for Rust.

[![Crates.io][crates-badge]][crates-url]
[![Github pages][gh-pages-badge]][gh-pages-url]
[![MIT licensed][mit-badge]][mit-url]
[![CI][actions-badge]][actions-url]
[![pre-commit][pre-commit-badge]][pre-commit-url]

[crates-badge]: https://img.shields.io/crates/v/dsalgo.svg
[crates-url]: https://crates.io/crates/dsalgo
[gh-pages-badge]: https://github.com/kagemeka/dsalgo_rust/actions/workflows/pages/pages-build-deployment/badge.svg
[gh-pages-url]: https://kagemeka.github.io/dsalgo_rust
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/kagemeka/dsalgo_rust/blob/main/LICENSE
[actions-badge]: https://github.com/kagemeka/dsalgo_rust/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/kagemeka/dsalgo_rust/actions/workflows/rust.yml
[pre-commit-badge]: https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white
[pre-commit-url]: https://github.com/pre-commit/pre-commit

## Development

### setup & CI

```sh
./ci.sh
source ~/.bashrc
```

### VSCode Extensions

- rust-analyzer

### see document

- <https://doc.rust-lang.org/cargo/commands/cargo-doc.html>

```sh
cargo doc --open
```

### clean targets

- <https://doc.rust-lang.org/cargo/commands/cargo-clean.html>
- use when something is wrong.

```sh
cargo clean
```

### check easy compilation errors

- <https://doc.rust-lang.org/cargo/commands/cargo-check.html>

```sh
cargo check
```
