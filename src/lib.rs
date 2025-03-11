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

// Include the state module
pub mod state;

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

/// Calculator module providing basic arithmetic operations.
///
/// This module contains functions for addition, subtraction, multiplication,
/// and division, all exposed to JavaScript through WebAssembly bindings.
pub mod calculator {
    use wasm_bindgen::prelude::*;

    /// Result type for calculator operations that might fail.
    ///
    /// This type is used for operations like division that can result in errors.
    pub type CalcResult = Result<f64, String>;

    // Memory storage for calculator operations
    thread_local! {
        static MEMORY: std::cell::RefCell<f64> = std::cell::RefCell::new(0.0);
    }

    /// Stores a value in the calculator's memory.
    ///
    /// This function overwrites any previous value stored in memory.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to store in memory
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wasm_calc::calculator::{memory_store, memory_recall};
    ///
    /// memory_store(42.0);
    /// assert_eq!(memory_recall(), 42.0);
    /// ```
    #[wasm_bindgen]
    pub fn memory_store(value: f64) {
        MEMORY.with(|memory| {
            *memory.borrow_mut() = value;
        });
    }

    /// Recalls the current value stored in the calculator's memory.
    ///
    /// # Returns
    ///
    /// The current value stored in memory
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wasm_calc::calculator::{memory_store, memory_recall};
    ///
    /// memory_store(42.0);
    /// assert_eq!(memory_recall(), 42.0);
    /// ```
    #[wasm_bindgen]
    pub fn memory_recall() -> f64 {
        MEMORY.with(|memory| {
            *memory.borrow()
        })
    }

    /// Clears the calculator's memory by setting it to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wasm_calc::calculator::{memory_store, memory_clear, memory_recall};
    ///
    /// memory_store(42.0);
    /// memory_clear();
    /// assert_eq!(memory_recall(), 0.0);
    /// ```
    #[wasm_bindgen]
    pub fn memory_clear() {
        MEMORY.with(|memory| {
            *memory.borrow_mut() = 0.0;
        });
    }

    /// Adds a value to the current value stored in memory.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the memory
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wasm_calc::calculator::{memory_clear, memory_add, memory_recall};
    ///
    /// memory_clear();
    /// memory_add(5.0);
    /// memory_add(3.0);
    /// assert_eq!(memory_recall(), 8.0);
    /// ```
    #[wasm_bindgen]
    pub fn memory_add(value: f64) {
        MEMORY.with(|memory| {
            *memory.borrow_mut() += value;
        });
    }

    /// Subtracts a value from the current value stored in memory.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to subtract from the memory
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wasm_calc::calculator::{memory_store, memory_subtract, memory_recall};
    ///
    /// memory_store(10.0);
    /// memory_subtract(3.0);
    /// assert_eq!(memory_recall(), 7.0);
    /// ```
    #[wasm_bindgen]
    pub fn memory_subtract(value: f64) {
        MEMORY.with(|memory| {
            *memory.borrow_mut() -= value;
        });
    }

    /// Adds two numbers and returns the result.
    ///
    /// # Arguments
    ///
    /// * `a` - First operand
    /// * `b` - Second operand
    ///
    /// # Returns
    ///
    /// The sum of `a` and `b`
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wasm_calc::calculator::add;
    ///
    /// assert_eq!(add(2.0, 3.0), 5.0);
    /// assert_eq!(add(-1.0, 1.0), 0.0);
    /// ```
    #[wasm_bindgen]
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    /// Subtracts the second number from the first and returns the result.
    ///
    /// # Arguments
    ///
    /// * `a` - First operand
    /// * `b` - Second operand
    ///
    /// # Returns
    ///
    /// The difference `a - b`
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wasm_calc::calculator::subtract;
    ///
    /// assert_eq!(subtract(5.0, 3.0), 2.0);
    /// assert_eq!(subtract(1.0, 1.0), 0.0);
    /// ```
    #[wasm_bindgen]
    pub fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    /// Multiplies two numbers and returns the result.
    ///
    /// # Arguments
    ///
    /// * `a` - First operand
    /// * `b` - Second operand
    ///
    /// # Returns
    ///
    /// The product of `a` and `b`
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wasm_calc::calculator::multiply;
    ///
    /// assert_eq!(multiply(2.0, 3.0), 6.0);
    /// assert_eq!(multiply(0.0, 5.0), 0.0);
    /// ```
    #[wasm_bindgen]
    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    /// Divides the first number by the second and returns the result.
    ///
    /// # Arguments
    ///
    /// * `a` - Dividend (numerator)
    /// * `b` - Divisor (denominator)
    ///
    /// # Returns
    ///
    /// * `Ok(result)` - The quotient `a / b` if successful
    /// * `Err(message)` - An error message if division by zero is attempted
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wasm_calc::calculator::divide;
    ///
    /// assert_eq!(divide(6.0, 3.0), Ok(2.0));
    /// assert_eq!(divide(1.0, 0.0).is_err(), true);
    /// ```
    #[wasm_bindgen]
    pub fn divide(a: f64, b: f64) -> CalcResult {
        if b == 0.0 {
            Err("Division by zero is not allowed".to_string())
        } else {
            Ok(a / b)
        }
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
}

/// Tests for calculator operations.
#[cfg(test)]
mod calculator_tests {
    use super::calculator::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    /// Tests for the add function.
    mod add_tests {
        use super::*;

        /// Tests basic addition with positive numbers.
        #[test]
        fn test_add_positive_numbers() {
            // Arrange
            let a = 2.0;
            let b = 3.0;
            
            // Act
            let result = add(a, b);
            
            // Assert
            assert_eq!(result, 5.0);
        }

        /// Tests addition with negative numbers.
        #[test]
        fn test_add_negative_numbers() {
            // Arrange
            let a = -2.0;
            let b = -3.0;
            
            // Act
            let result = add(a, b);
            
            // Assert
            assert_eq!(result, -5.0);
        }

        /// Tests addition with mixed positive and negative numbers.
        #[test]
        fn test_add_mixed_signs() {
            // Arrange
            let a = 5.0;
            let b = -3.0;
            
            // Act
            let result = add(a, b);
            
            // Assert
            assert_eq!(result, 2.0);
        }

        /// Tests addition with zero.
        #[test]
        fn test_add_with_zero() {
            // Arrange
            let a = 5.0;
            let b = 0.0;
            
            // Act
            let result = add(a, b);
            
            // Assert
            assert_eq!(result, 5.0);
        }

        /// Tests addition with floating point numbers.
        #[test]
        fn test_add_floating_point() {
            // Arrange
            let a = 0.1;
            let b = 0.2;
            
            // Act
            let result = add(a, b);
            
            // Assert
            // Use a larger epsilon for floating point comparison
            const EPSILON: f64 = 1e-10;
            assert!((result - 0.3).abs() < EPSILON);
        }

        /// Tests addition in WebAssembly environment.
        #[wasm_bindgen_test]
        fn test_add_wasm() {
            // Arrange
            let a = 10.0;
            let b = 20.0;
            
            // Act
            let result = add(a, b);
            
            // Assert
            assert_eq!(result, 30.0);
        }
    }

    /// Tests for the subtract function.
    mod subtract_tests {
        use super::*;

        /// Tests basic subtraction with positive numbers.
        #[test]
        fn test_subtract_positive_numbers() {
            // Arrange
            let a = 5.0;
            let b = 3.0;
            
            // Act
            let result = subtract(a, b);
            
            // Assert
            assert_eq!(result, 2.0);
        }

        /// Tests subtraction with negative numbers.
        #[test]
        fn test_subtract_negative_numbers() {
            // Arrange
            let a = -2.0;
            let b = -3.0;
            
            // Act
            let result = subtract(a, b);
            
            // Assert
            assert_eq!(result, 1.0);
        }

        /// Tests subtraction with mixed positive and negative numbers.
        #[test]
        fn test_subtract_mixed_signs() {
            // Arrange
            let a = 5.0;
            let b = -3.0;
            
            // Act
            let result = subtract(a, b);
            
            // Assert
            assert_eq!(result, 8.0);
        }

        /// Tests subtraction with zero.
        #[test]
        fn test_subtract_with_zero() {
            // Arrange
            let a = 5.0;
            let b = 0.0;
            
            // Act
            let result = subtract(a, b);
            
            // Assert
            assert_eq!(result, 5.0);
        }

        /// Tests subtraction with floating point numbers.
        #[test]
        fn test_subtract_floating_point() {
            // Arrange
            let a = 0.3;
            let b = 0.1;
            
            // Act
            let result = subtract(a, b);
            
            // Assert
            // Use a larger epsilon for floating point comparison
            const EPSILON: f64 = 1e-10;
            assert!((result - 0.2).abs() < EPSILON);
        }

        /// Tests subtraction in WebAssembly environment.
        #[wasm_bindgen_test]
        fn test_subtract_wasm() {
            // Arrange
            let a = 30.0;
            let b = 10.0;
            
            // Act
            let result = subtract(a, b);
            
            // Assert
            assert_eq!(result, 20.0);
        }
    }

    /// Tests for the multiply function.
    mod multiply_tests {
        use super::*;

        /// Tests basic multiplication with positive numbers.
        #[test]
        fn test_multiply_positive_numbers() {
            // Arrange
            let a = 2.0;
            let b = 3.0;
            
            // Act
            let result = multiply(a, b);
            
            // Assert
            assert_eq!(result, 6.0);
        }

        /// Tests multiplication with negative numbers.
        #[test]
        fn test_multiply_negative_numbers() {
            // Arrange
            let a = -2.0;
            let b = -3.0;
            
            // Act
            let result = multiply(a, b);
            
            // Assert
            assert_eq!(result, 6.0);
        }

        /// Tests multiplication with mixed positive and negative numbers.
        #[test]
        fn test_multiply_mixed_signs() {
            // Arrange
            let a = 5.0;
            let b = -3.0;
            
            // Act
            let result = multiply(a, b);
            
            // Assert
            assert_eq!(result, -15.0);
        }

        /// Tests multiplication with zero.
        #[test]
        fn test_multiply_with_zero() {
            // Arrange
            let a = 5.0;
            let b = 0.0;
            
            // Act
            let result = multiply(a, b);
            
            // Assert
            assert_eq!(result, 0.0);
        }

        /// Tests multiplication with floating point numbers.
        #[test]
        fn test_multiply_floating_point() {
            // Arrange
            let a = 0.1;
            let b = 0.2;
            
            // Act
            let result = multiply(a, b);
            
            // Assert
            // Use a larger epsilon for floating point comparison
            const EPSILON: f64 = 1e-10;
            assert!((result - 0.02).abs() < EPSILON);
        }

        /// Tests multiplication in WebAssembly environment.
        #[wasm_bindgen_test]
        fn test_multiply_wasm() {
            // Arrange
            let a = 10.0;
            let b = 20.0;
            
            // Act
            let result = multiply(a, b);
            
            // Assert
            assert_eq!(result, 200.0);
        }
    }

    /// Tests for the divide function.
    mod divide_tests {
        use super::*;

        /// Tests basic division with positive numbers.
        #[test]
        fn test_divide_positive_numbers() {
            // Arrange
            let a = 6.0;
            let b = 3.0;
            
            // Act
            let result = divide(a, b);
            
            // Assert
            assert_eq!(result.unwrap(), 2.0);
        }

        /// Tests division with negative numbers.
        #[test]
        fn test_divide_negative_numbers() {
            // Arrange
            let a = -6.0;
            let b = -3.0;
            
            // Act
            let result = divide(a, b);
            
            // Assert
            assert_eq!(result.unwrap(), 2.0);
        }

        /// Tests division with mixed positive and negative numbers.
        #[test]
        fn test_divide_mixed_signs() {
            // Arrange
            let a = 6.0;
            let b = -3.0;
            
            // Act
            let result = divide(a, b);
            
            // Assert
            assert_eq!(result.unwrap(), -2.0);
        }

        /// Tests division by zero.
        #[test]
        fn test_divide_by_zero() {
            // Arrange
            let a = 5.0;
            let b = 0.0;
            
            // Act
            let result = divide(a, b);
            
            // Assert
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Division by zero is not allowed");
        }

        /// Tests division with floating point numbers.
        #[test]
        fn test_divide_floating_point() {
            // Arrange
            let a = 0.6;
            let b = 0.2;
            
            // Act
            let result = divide(a, b);
            
            // Assert
            // Use a larger epsilon for floating point comparison
            const EPSILON: f64 = 1e-10;
            assert!((result.unwrap() - 3.0).abs() < EPSILON);
        }

        /// Tests division in WebAssembly environment.
        #[wasm_bindgen_test]
        fn test_divide_wasm() {
            // Arrange
            let a = 20.0;
            let b = 10.0;
            
            // Act
            let result = divide(a, b);
            
            // Assert
            assert_eq!(result.unwrap(), 2.0);
        }

        /// Tests division by zero in WebAssembly environment.
        #[wasm_bindgen_test]
        fn test_divide_by_zero_wasm() {
            // Arrange
            let a = 20.0;
            let b = 0.0;
            
            // Act
            let result = divide(a, b);
            
            // Assert
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Division by zero is not allowed");
        }
    }

    /// Tests for memory operations.
    mod memory_tests {
        use super::*;

        /// Tests storing a value in memory.
        #[test]
        fn test_memory_store() {
            // Arrange
            let value = 42.0;
            
            // Act
            memory_store(value);
            let result = memory_recall();
            
            // Assert
            assert_eq!(result, 42.0);
        }

        /// Tests recalling a value from memory.
        #[test]
        fn test_memory_recall() {
            // Arrange
            memory_store(10.0);
            
            // Act
            let result = memory_recall();
            
            // Assert
            assert_eq!(result, 10.0);
        }

        /// Tests clearing the memory.
        #[test]
        fn test_memory_clear() {
            // Arrange
            memory_store(10.0);
            
            // Act
            memory_clear();
            let result = memory_recall();
            
            // Assert
            assert_eq!(result, 0.0);
        }

        /// Tests adding to memory.
        #[test]
        fn test_memory_add() {
            // Arrange
            memory_store(10.0);
            
            // Act
            memory_add(5.0);
            let result = memory_recall();
            
            // Assert
            assert_eq!(result, 15.0);
        }

        /// Tests subtracting from memory.
        #[test]
        fn test_memory_subtract() {
            // Arrange
            memory_store(10.0);
            
            // Act
            memory_subtract(3.0);
            let result = memory_recall();
            
            // Assert
            assert_eq!(result, 7.0);
        }

        /// Tests memory operations in sequence.
        #[test]
        fn test_memory_sequence() {
            // Arrange & Act
            memory_clear();
            memory_add(5.0);
            memory_add(10.0);
            memory_subtract(3.0);
            let result = memory_recall();
            
            // Assert
            assert_eq!(result, 12.0);
        }

        /// Tests memory operations in WebAssembly environment.
        #[wasm_bindgen_test]
        fn test_memory_wasm() {
            // Arrange & Act
            memory_clear();
            memory_store(20.0);
            memory_add(5.0);
            memory_subtract(10.0);
            let result = memory_recall();
            
            // Assert
            assert_eq!(result, 15.0);
        }
    }
}
