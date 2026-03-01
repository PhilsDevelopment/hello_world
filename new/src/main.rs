//! # Hello World Rust Project
//!
//! A complete, well-commented "Hello World" example in Rust,
//! demonstrating module structure, functions, and unit tests.

// Import the greeter module from our library crate (lib.rs).
// Now that lib.rs owns the module declaration, main.rs references
// it through the crate name rather than redeclaring it.
use hello_world::greeter;

/// Entry point of the application.
///
/// Rust programs always begin execution at `main()`.
/// The `fn` keyword declares a function.
fn main() {
    // Call our greeting function from the greeter module and bind the result
    let message = greeter::build_greeting("World");

    // Print the greeting to standard output.
    // The `!` after `println` means it's a macro, not a regular function.
    println!("{}", message);
}
