# Hello World — Rust

A complete, well-commented "Hello World" project in Rust demonstrating:

- Project structure with multiple source files (modules)
- Explicit inline and doc comments
- Unit tests with `#[cfg(test)]`
- Git version control

## Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (install via `rustup`)

## Project Structure

```
hello_world_rust/
├── Cargo.toml      # Package manifest & dependencies
├── .gitignore      # Files excluded from Git
├── README.md       # This file
└── src/
    ├── main.rs     # Entry point
    └── greeter.rs  # Greeting logic & unit tests
```

## Run

```bash
cargo run
# Output: Hello, World!
```

## Test

```bash
cargo test
```

All 6 unit tests should pass.

## Build (release)

```bash
cargo build --release
# Binary at: ./target/release/hello_world_rust
```
