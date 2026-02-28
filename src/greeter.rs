//! # Greeter Module
//!
//! This module contains greeting-related logic, separated from `main.rs`
//! to demonstrate Rust's module system and make the code testable.

/// Constructs a greeting string for the given name.
///
/// # Arguments
///
/// * `name` - A string slice (`&str`) representing the name to greet.
///
/// # Returns
///
/// A `String` containing the full greeting message.
///
/// # Examples
///
/// ```
/// let msg = hello_world_rust::greeter::build_greeting("Alice");
/// assert_eq!(msg, "Hello, Alice!");
/// ```
pub fn build_greeting(name: &str) -> String {
    // The `format!` macro works like `println!` but returns a String
    // instead of printing to stdout. The `{}` is a placeholder for `name`.
    format!("Hello, {}!", name)
}

/// Returns whether a name is considered "empty" (blank or whitespace-only).
///
/// # Arguments
///
/// * `name` - A string slice to check.
///
/// # Returns
///
/// `true` if the name is empty or only whitespace; `false` otherwise.
pub fn is_empty_name(name: &str) -> bool {
    // `.trim()` removes leading/trailing whitespace.
    // `.is_empty()` returns true if the resulting slice has no characters.
    name.trim().is_empty()
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
        let result = build_greeting("World");
        // `assert_eq!` panics (fails the test) if the two values are not equal.
        assert_eq!(result, "Hello, World!");
    }

    /// Test that a different name also produces the correct greeting.
    #[test]
    fn test_greeting_with_different_name() {
        let result = build_greeting("Alice");
        assert_eq!(result, "Hello, Alice!");
    }

    /// Test that an empty string still produces a valid (though odd) greeting.
    #[test]
    fn test_greeting_with_empty_string() {
        let result = build_greeting("");
        assert_eq!(result, "Hello, !");
    }

    /// Test that `is_empty_name` correctly identifies an empty string.
    #[test]
    fn test_is_empty_name_with_empty_string() {
        // `assert!` panics if the expression is false.
        assert!(is_empty_name(""));
    }

    /// Test that `is_empty_name` correctly identifies whitespace-only input.
    #[test]
    fn test_is_empty_name_with_whitespace() {
        assert!(is_empty_name("   "));
    }

    /// Test that `is_empty_name` returns false for a real name.
    #[test]
    fn test_is_empty_name_with_real_name() {
        assert!(!is_empty_name("Bob"));
    }
}
