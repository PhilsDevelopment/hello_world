# Hello World — Rust

[![Repository](https://img.shields.io/badge/repo-GitHub-blue)](https://github.com/PhillipSpencerHunt/hello_world)
[![Language](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org)

A comprehensive, production-ready "Hello World" project in Rust demonstrating best practices and core concepts.

## Features

- **Modular Architecture:** Project split across multiple source files (`main.rs`, `greeter.rs`, `lib.rs`) with proper module organization
- **Well-Documented Code:** Explicit inline comments and doc comments for clarity
- **Unit Tests:** `#[cfg(test)]` modules with comprehensive test coverage (~6 tests)
- **Integration Tests:** Dedicated `tests/` directory with integration test suites
- **Version Control:** Git-ready with `.gitignore` configured
- **Release-Ready Build:** Optimized release build configuration via `Cargo.toml`

## Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (install via `rustup`)
- Git (for cloning and version control)

## Project Structure

```
hello_world/
├── Cargo.toml                          # Package manifest & dependencies
├── Cargo.lock                          # Locked dependency versions
├── .gitignore                          # Files excluded from Git
├── README.md                           # This file
├── src/
│   ├── main.rs                         # Entry point and CLI logic
│   ├── lib.rs                          # Library exports
│   └── greeter.rs                      # Greeting module with unit tests
└── tests/
    └── greeter_integration_tests.rs    # Integration tests for greeter module
```

## Quick Start

### Run

```bash
cargo run
```

**Expected output:**
```
Hello, World!
```

### Run Tests

Execute all unit and integration tests:

```bash
cargo test
```

All tests should pass (unit tests in modules + integration tests).

### Build (Release)

Create an optimized release binary:

```bash
cargo build --release
```

**Binary location:** `./target/release/hello_world`

## Development

### Check Code

Lint and format check without building:

```bash
cargo check
cargo fmt --check
```

### Format Code

Auto-format all source files:

```bash
cargo fmt
```

### Watch Mode

Continuously build and test on file changes:

```bash
cargo watch -x test
```

(Requires `cargo-watch`: `cargo install cargo-watch`)

## Repository

**GitHub:** [PhillipSpencerHunt/hello_world](https://github.com/PhillipSpencerHunt/hello_world)

Clone and contribute:

```bash
git clone git@github.com:PhillipSpencerHunt/hello_world.git
cd hello_world
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit changes with clear messages
4. Push to your branch and open a Pull Request

## License

This project is provided as-is for educational purposes. Customize the license as needed.

## Author

**Phillip Spencer Hunt**

Questions or feedback? Open an issue on [GitHub](https://github.com/PhillipSpencerHunt/hello_world/issues).
