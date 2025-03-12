//! Rust WASM Calculator is a WebAssembly calculator built with Rust.
//!
//! This crate provides a calculator implementation that can be used in web browsers
//! through WebAssembly. It follows test-driven development practices and provides
//! a clean, type-safe interface for basic and advanced calculations.
//!
//! # Features
//!
//! - WebAssembly integration using wasm-bindgen
//! - Basic arithmetic operations
//! - Memory functions
//! - State management for calculator operations
//! - Error handling with proper JavaScript integration
//!
//! # Examples
//!
//! ```rust
//! use rust_wasm_calc::hello;
//!
//! // Basic greeting functionality
//! let greeting = hello("Calculator");
//! assert_eq!(greeting, "Hello, Calculator!");
//! ```

use wasm_bindgen::prelude::*;

// Include modules
pub mod calculator;
pub mod state;
pub mod errors;

/// Returns a friendly greeting message.
///
/// This function is currently a placeholder that demonstrates the WebAssembly
/// binding functionality. It will be replaced with calculator operations
/// in future versions.
///
/// # Arguments
///
/// * `name` - A string slice that contains the name to greet
///
/// # Examples
///
/// ```rust
/// use rust_wasm_calc::hello;
///
/// let result = hello("World");
/// assert_eq!(result, "Hello, World!");
///
/// // Works with empty strings
/// let empty = hello("");
/// assert_eq!(empty, "Hello, !");
/// ```
///
/// # Note
///
/// This function is exposed to JavaScript through WebAssembly and can be
/// called from web applications.
#[wasm_bindgen]
pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Tests for the hello function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_rust() {
        assert_eq!(hello("Rust"), "Hello, Rust!");
    }

    #[test]
    fn test_hello_wasm_browser() {
        assert_eq!(hello("WebAssembly"), "Hello, WebAssembly!");
    }

    #[test]
    fn test_hello_empty_string() {
        assert_eq!(hello(""), "Hello, !");
    }

    #[test]
    fn test_hello_long_string() {
        let long_name = "a".repeat(1000);
        let expected = format!("Hello, {}!", long_name);
        assert_eq!(hello(&long_name), expected);
    }
}

// The calculator tests are now moved to their respective modules
// in tests/calculator/operations_tests.rs and tests/calculator/memory_tests.rs
