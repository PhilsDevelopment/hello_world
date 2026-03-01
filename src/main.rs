//! # Hello World Rust Project
//!
//! A complete, well-commented "Hello World" example in Rust,
//! demonstrating module structure, functions, and unit tests.

// Declare the `greeter` module, defined in greeter.rs
mod greeter;

/// Entry point of the application.
///
/// Rust programs always begin execution at `main()`.
/// The `fn` keyword declares a function.
fn main() {
  // Create a greeter instance.
  let mut greeter = greeter::Greeter::new("World");
  // Call our greeting function from the greeter module and bind the result
  // let message = greeter::build_greeting("World");

  if !greeter.has_greeted() {
    // Call the greet funtion.
    greeter.greet();
  }

  // // Print the grecargo run greeter string empty: {}", message_empty);
}
