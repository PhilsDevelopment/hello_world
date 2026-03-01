//! # Greeter Module
//!
//! This module contains greeting-related logic, separated from `main.rs`
//! to demonstrate Rust's module system and make the code testable.

// Define the struct with its "member variables" (called fields in Rust)
pub struct Greeter {
  name: String,       // private field (no 'pub')
  greet_called: bool, // private field (no 'pub')
}

// Methods go in an `impl` block — equivalent to a class's method definitions
impl Greeter {
  /// Constructs a Greeter object with the given name (by convention named `new`).
  ///
  /// # Arguments
  ///
  /// * `name` - A string slice (`&str`) representing the name to greet.
  ///
  /// # Examples
  ///
  /// ```
  /// let mut grt = hello_world::greeter::Greeter::new("Alice");
  /// ```
  pub fn new(name: &str) -> Greeter {
    Greeter { name: name.to_string(), greet_called: false }
  }

  /// Get the greeting string of the greeter.
  /// # Returns
  ///
  /// A `String` containing the full greeting message.
  ///
  pub fn get_greeting(&self) -> String {
    if self.is_empty_name() {
      "Hello!".to_string()
    } else {
      format!("Hello, {}!", self.name)
    }
  }

  // Method that uses the struct's fields via `self`
  pub fn greet(&mut self) {
    // Print the greeting to standard output.
    // The `!` after `println` means it's a macro, not a regular function.
    println!("{}", self.get_greeting());
    self.greet_called = true;
  }

  /// Returns whether greet has been called.
  ///
  /// # Returns
  ///
  /// `true` if greet has not been called; `false` otherwise.
  pub fn has_greeted(&self) -> bool {
    self.greet_called
  }

  /// Returns whether the name is considered "empty" (blank or whitespace-only).
  ///
  /// # Returns
  ///
  /// `true` if the name is empty or only whitespace; `false` otherwise.
  fn is_empty_name(&self) -> bool {
    // `.trim()` removes leading/trailing whitespace.
    // `.is_empty()` returns true if the resulting slice has no characters.
    self.name.trim().is_empty()
  }
}

// ============================================================
//  Unit Tests
//
//  In Rust, tests live in the same file as the code they test,
//  inside a `#[cfg(test)]` module. This annotation tells the
//  compiler to include this block ONLY when running `cargo test`.
// ============================================================
#[cfg(test)]
mod tests {
  // `use super::*` brings all items from the parent module (greeter)
  // into scope so we can call `build_greeting` directly.
  use super::*;

  /// Test that a standard name produces the expected greeting.
  #[test]
  fn test_greeting_with_name() {
    let uut = Greeter::new("World");
    let result = uut.get_greeting();

    // `assert_eq!` panics (fails the test) if the two values are not equal.
    assert_eq!(result, "Hello, World!");
  }

  /// Test that a different name also produces the correct greeting.
  #[test]
  fn test_greeting_with_different_name() {
    let uut = Greeter::new("Alice");
    let result = uut.get_greeting();
    assert_eq!(result, "Hello, Alice!");
  }

  /// Test that an empty string still produces a valid (though odd) greeting.
  #[test]
  fn test_greeting_with_empty_string() {
    let uut = Greeter::new("");
    let result = uut.get_greeting();
    assert_eq!(result, "Hello!");
  }

  /// Test that the greet function calls only once.
  #[test]
  fn test_greet() {
    let mut uut = Greeter::new("");
    let result = uut.has_greeted();

    // `assert!` panics if the expression is false.
    assert!(!result);

    // greet.
    uut.greet();

    let result = uut.has_greeted();
    assert!(result);
  }
}
