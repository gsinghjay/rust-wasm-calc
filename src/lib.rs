//! Rust WASM Calculator is a WebAssembly calculator built with Rust.
//!
//! This crate provides a calculator implementation that can be used in web browsers
//! through WebAssembly. It follows test-driven development practices and provides
//! a clean, type-safe interface for basic and advanced calculations.
//!
//! # Features
//!
//! - WebAssembly integration using wasm-bindgen
//! - Basic arithmetic operations (coming soon)
//! - Memory functions (planned)
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

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    /// Tests the hello function with a basic Rust string.
    #[test]
    fn test_hello_rust() {
        let result = hello("Rust");
        assert_eq!(result, "Hello, Rust!");
    }

    /// Tests the hello function in a browser WebAssembly environment.
    #[wasm_bindgen_test]
    fn test_hello_wasm_browser() {
        let result = hello("WASM");
        assert_eq!(result, "Hello, WASM!");
    }

    /// Tests the hello function with an empty string.
    #[wasm_bindgen_test]
    fn test_hello_empty_string() {
        let result = hello("");
        assert_eq!(result, "Hello, !");
    }

    /// Tests the hello function with a very long input string.
    /// This test ensures the function handles large inputs correctly.
    #[wasm_bindgen_test]
    fn test_hello_long_string() {
        let long_name = "a".repeat(1000);
        let result = hello(&long_name);
        assert!(result.starts_with("Hello, "));
        assert!(result.ends_with("!"));
    }
}
