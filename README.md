[![Build and Test](https://github.com/ImperialBower/wincounter/actions/workflows/basic.yaml/badge.svg)](https://github.com/ImperialBower/wincounter/actions/workflows/basic.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/wincounter.svg)](https://crates.io/crates/wincounter)
[![Rustdocs](https://docs.rs/wincounter/badge.svg)](https://docs.rs/wincounter/)
[![WASM](https://img.shields.io/badge/WASM-compatible-darkblue)](https://webassembly.org/)

# Win Counter

The library is designed to address a very specific problem found with reporting
the results in games: _How to do clearly store and report win percentages when
you have x number of players, and they can each tie with any number of other 
players?_

## Setup

This program uses [cargo make](https://github.com/sagiegurari/cargo-make) to manage tasks. Install it with:

```shell
cargo install cargo-make
```

The default `cargo make` runs the following tasks:

* `cargo fmt`
* `cargo clean`
* `cargo build`
* `cargo test`
* `cargo clippy` with `clippy::pedantic` lint settings
* `cargo check --target wasm32-unknown-unknown` (WASM compatibility)
* `cargo doc --no-deps`

```shell
❯ cargo make
````

To open the generated docs in your browser:

```shell
❯ cargo make docs
```
