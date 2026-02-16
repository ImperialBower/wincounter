# WASM Compatibility

This crate is compatible with WebAssembly (WASM) targets.

## Testing WASM Compatibility

To verify that the crate compiles for WASM, run:

```shell
cargo check --target wasm32-unknown-unknown
```

Or using cargo-make:

```shell
cargo make wasm
```

## CI/CD

The GitHub Actions CI workflow includes a WASM compatibility check that runs on every push and pull request.

## Limitations

When targeting WASM, the following functions are **not available** because they rely on file system access:

- `util::Util::read_lines()` - This function is gated behind `#[cfg(not(target_arch = "wasm32"))]`

All other functionality in the crate works normally in WASM environments.

## Usage in WASM Projects

You can use this crate in WASM projects by adding it to your `Cargo.toml`:

```toml
[dependencies]
wincounter = "0.1"
```

The crate will automatically exclude file I/O functions when compiled for WASM targets.

## Example WASM Usage

```rust
use wincounter::heads_up::HeadsUp;
use wincounter::wins::Wins;
use wincounter::win::Win;

// All core functionality works in WASM
let hand = HeadsUp::new(1_365_284, 314_904, 32_116);
println!("Player 1 win percentage: {:.2}%", hand.percentage_first());

let mut wins = Wins::default();
wins.add(Win::FIRST);
wins.add(Win::SECOND);
wins.add(Win::FIRST | Win::SECOND);
```

## Building for WASM

If you need to add the WASM target to your Rust toolchain:

```shell
rustup target add wasm32-unknown-unknown
```

Then build:

```shell
cargo build --target wasm32-unknown-unknown
```

