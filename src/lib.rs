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
/// This function demonstrates the WebAssembly binding functionality.
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

/// Adds two numbers together.
///
/// # Arguments
///
/// * `a` - First number
/// * `b` - Second number
///
/// # Examples
///
/// ```rust
/// use rust_wasm_calc::add;
///
/// let result = add(2.5, 3.7);
/// assert_eq!(result, 6.2);
/// ```
#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts the second number from the first.
///
/// # Arguments
///
/// * `a` - Number to subtract from
/// * `b` - Number to subtract
///
/// # Examples
///
/// ```rust
/// use rust_wasm_calc::subtract;
///
/// let result = subtract(5.0, 3.0);
/// assert_eq!(result, 2.0);
/// ```
#[wasm_bindgen]
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies two numbers together.
///
/// # Arguments
///
/// * `a` - First number
/// * `b` - Second number
///
/// # Examples
///
/// ```rust
/// use rust_wasm_calc::multiply;
///
/// let result = multiply(4.0, 2.5);
/// assert_eq!(result, 10.0);
/// ```
#[wasm_bindgen]
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides the first number by the second.
///
/// Returns NaN if attempting to divide by zero.
///
/// # Arguments
///
/// * `a` - Dividend
/// * `b` - Divisor
///
/// # Examples
///
/// ```rust
/// use rust_wasm_calc::divide;
///
/// let result = divide(10.0, 2.0);
/// assert_eq!(result, 5.0);
///
/// let zero_division = divide(1.0, 0.0);
/// assert!(zero_division.is_nan());
/// ```
#[wasm_bindgen]
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        std::f64::NAN
    } else {
        a / b
    }
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

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(add(-1.0, 1.0), 0.0);
        assert_eq!(add(0.1, 0.2), 0.3);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
        assert_eq!(subtract(1.0, 1.0), 0.0);
        assert_eq!(subtract(-1.0, -1.0), 0.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4.0, 2.0), 8.0);
        assert_eq!(multiply(-2.0, 3.0), -6.0);
        assert_eq!(multiply(0.0, 5.0), 0.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6.0, 2.0), 3.0);
        assert_eq!(divide(-6.0, 2.0), -3.0);
        assert!(divide(1.0, 0.0).is_nan());
    }

    // WASM-specific tests
    #[wasm_bindgen_test]
    fn test_calculator_operations_wasm() {
        assert_eq!(add(2.0, 2.0), 4.0);
        assert_eq!(subtract(5.0, 3.0), 2.0);
        assert_eq!(multiply(3.0, 4.0), 12.0);
        assert_eq!(divide(10.0, 2.0), 5.0);
        assert!(divide(1.0, 0.0).is_nan());
    }
}
