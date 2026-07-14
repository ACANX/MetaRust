# MetaRust

A Rust utility library providing common helpers and tools.

## Features

- **Rust 2021 edition** — modern Rust with strict type safety
- **Zero dependencies** — lightweight and fast
- **Well-tested** — comprehensive test coverage

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
metarust = { git = "https://github.com/ACANX/MetaRust" }
```

```rust
use metarust::greet;

let message = greet("World");
println!("{}", message);
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic

# Build release
cargo build --release
```

## License

Apache License 2.0
