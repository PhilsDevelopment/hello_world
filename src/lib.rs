//! # hello_world Library
//!
//! This file exposes the project's modules as a library crate so that
//! integration tests (in `tests/`) and external code can import them.
//!
//! Rust projects can be both a binary (`main.rs`) AND a library (`lib.rs`)
//! at the same time — Cargo handles both automatically.

// Re-export the greeter module so callers can use:
//   use hello_world::greeter;
pub mod greeter;
