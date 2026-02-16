# Quick WASM Compatibility Reference

## ✅ What Works in WASM

- All core types: `HeadsUp`, `Wins`, `Results`, `Win`
- All percentage calculations
- Bit flag operations for player tracking
- Serde serialization/deserialization
- All utility functions except file I/O

## ❌ What Doesn't Work in WASM

- `util::Util::read_lines()` - File system access not available

## 🧪 Testing WASM Compatibility

```bash
# Quick check
cargo check --target wasm32-unknown-unknown

# Using cargo-make
cargo make wasm

# Full build + test script
bin/checkwasm
```

## 📦 Adding to Your WASM Project

```toml
[dependencies]
wincounter = "0.1"
```

That's it! No special features or configuration needed.

## 🔍 CI/CD

WASM compatibility is automatically tested in GitHub Actions on every push/PR.

## 📚 More Info

- See `WASM.md` for detailed usage guide
- See `WASM_IMPLEMENTATION.md` for implementation details

