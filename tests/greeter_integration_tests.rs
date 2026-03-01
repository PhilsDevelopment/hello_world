//! # Integration Tests — Greeter
//!
//! Integration tests live in the `tests/` folder at the project root.
//! Unlike unit tests (which are inside the module they test), integration
//! tests are compiled as a completely separate crate that imports your
//! library from the outside — exactly as an end user would.
//!
//! Run all tests with:
//!   cargo test
//!
//! Run only integration tests with:
//!   cargo test --test greeter_integration_tests

// Import the library crate by its package name (from Cargo.toml).
// This is the same `use` statement any external consumer would write.
use hello_world::greeter;

// ============================================================
//  build_greeting — integration tests
// ============================================================

/// Verify the happy path: a normal name produces the correct greeting.
#[test]
fn test_build_greeting_basic() {
    let greeter = greeter::Greeter::new("World");
    let result= greeter.get_greeting();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_build_greeting_no_name() {
    let greeter = greeter::Greeter::new("");
    let result= greeter.get_greeting();
    assert_eq!(result, "Hello!");
}

/// Verify that the greeting works with a multi-word name.
#[test]
fn test_build_greeting_full_name() {
    let greeter = greeter::Greeter::new("John Smith");
    let result= greeter.get_greeting();
    assert_eq!(result, "Hello, John Smith!");
}

/// Verify that Unicode names are handled correctly.
#[test]
fn test_build_greeting_unicode() {
    let greeter = greeter::Greeter::new("世界");
    let result= greeter.get_greeting();
    assert_eq!(result, "Hello, 世界!");
}

/// Verify that a name with leading/trailing whitespace is preserved as-is.
/// (Trimming is the caller's responsibility, not build_greeting's.)
#[test]
fn test_build_greeting_preserves_whitespace() {
    let greeter = greeter::Greeter::new("  Alice  ");
    let result= greeter.get_greeting();
    assert_eq!(result, "Hello,   Alice  !");
}

/// Verify that the return type is a proper owned String, not a slice.
/// This confirms the function allocates and returns new memory correctly.
#[test]
fn test_build_greeting_returns_owned_string() {
    let greeter = greeter::Greeter::new("Rust");
    let result= greeter.get_greeting();
    // If this compiles and passes, the return type is confirmed as String.
    assert!(!result.is_empty());
}

// ============================================================
//  is_empty_name — integration tests
// ============================================================

// /// Verify that an empty string is detected as empty.
// #[test]
// fn test_is_empty_name_empty_string() {
//     assert!(greeter::is_empty_name(""));
// }

// /// Verify that a string of only spaces is detected as empty.
// #[test]
// fn test_is_empty_name_spaces_only() {
//     assert!(greeter::is_empty_name("     "));
// }

// /// Verify that a string of mixed whitespace (tabs, newlines) is detected as empty.
// #[test]
// fn test_is_empty_name_mixed_whitespace() {
//     assert!(greeter::is_empty_name("\t\n\r"));
// }

// /// Verify that a real name is NOT considered empty.
// #[test]
// fn test_is_empty_name_valid_name() {
//     assert!(!greeter::is_empty_name("Alice"));
// }

// /// Verify that a name with surrounding whitespace is NOT considered empty
// /// (it has real content once trimmed).
// #[test]
// fn test_is_empty_name_name_with_padding() {
//     assert!(!greeter::is_empty_name("  Bob  "));
// }

// ============================================================
//  Combined behaviour tests
//
//  These tests exercise how the functions work together, which
//  is the primary purpose of integration tests — testing the
//  *interface between components*, not just individual units.
// ============================================================

/// A caller should check for empty names before building a greeting.
/// This test verifies the two functions compose correctly together.
#[test]
fn test_safe_greeting_workflow() {
    let name = "Alice";

    let mut uut = greeter::Greeter::new(name);
    let result= uut.has_greeted();

    // `assert!` panics if the expression is false.
    assert!(!result);

    // greet.
    uut.greet();

    let result = uut.has_greeted();
    assert!(result);
  
}

/// When given an empty name, the fallback greeting should be used.
#[test]
fn test_safe_greeting_workflow_empty_name() {
    let name = "";

        let mut uut = greeter::Greeter::new(name);
    let result= uut.has_greeted();

    // `assert!` panics if the expression is false.
    assert!(!result);

    // greet.
    uut.greet();

    let result = uut.has_greeted();
    assert!(result);
}
