//! Error handling module for the Rust WASM Calculator.
//!
//! This module provides standardized error types and conversion functions
//! for handling errors in calculator operations. It ensures consistent
//! error handling across the application and proper integration with JavaScript.

use wasm_bindgen::prelude::*;
use std::fmt;

/// Represents the different types of errors that can occur in calculator operations.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorErrorType {
    /// Division by zero error
    DivisionByZero,
    /// Invalid input error (e.g., non-numeric input)
    InvalidInput,
    /// Overflow error (result too large)
    Overflow,
    /// Underflow error (result too small)
    Underflow,
    /// General calculation error
    CalculationError,
}

/// Represents an error that occurred during a calculator operation.
#[derive(Debug, Clone)]
pub struct CalculatorError {
    /// The type of error that occurred
    error_type: CalculatorErrorType,
    /// A human-readable message describing the error
    message: String,
}

impl CalculatorError {
    /// Creates a new calculator error.
    ///
    /// # Arguments
    ///
    /// * `error_type` - The type of error that occurred
    /// * `message` - A human-readable message describing the error
    ///
    /// # Returns
    ///
    /// A new `CalculatorError` instance
    pub fn new(error_type: CalculatorErrorType, message: impl Into<String>) -> Self {
        Self {
            error_type,
            message: message.into(),
        }
    }

    /// Creates a division by zero error.
    ///
    /// # Returns
    ///
    /// A new `CalculatorError` instance with the `DivisionByZero` error type
    pub fn division_by_zero() -> Self {
        Self::new(
            CalculatorErrorType::DivisionByZero,
            "Division by zero is not allowed",
        )
    }

    /// Creates an invalid input error.
    ///
    /// # Arguments
    ///
    /// * `details` - Optional details about the invalid input
    ///
    /// # Returns
    ///
    /// A new `CalculatorError` instance with the `InvalidInput` error type
    pub fn invalid_input(details: Option<&str>) -> Self {
        let message = match details {
            Some(details) => format!("Invalid input: {}", details),
            None => "Invalid input".to_string(),
        };
        Self::new(CalculatorErrorType::InvalidInput, message)
    }

    /// Creates an overflow error.
    ///
    /// # Returns
    ///
    /// A new `CalculatorError` instance with the `Overflow` error type
    pub fn overflow() -> Self {
        Self::new(
            CalculatorErrorType::Overflow,
            "Result is too large to represent",
        )
    }

    /// Creates an underflow error.
    ///
    /// # Returns
    ///
    /// A new `CalculatorError` instance with the `Underflow` error type
    pub fn underflow() -> Self {
        Self::new(
            CalculatorErrorType::Underflow,
            "Result is too small to represent",
        )
    }

    /// Gets the error type.
    ///
    /// # Returns
    ///
    /// The type of error that occurred
    pub fn error_type(&self) -> CalculatorErrorType {
        self.error_type.clone()
    }

    /// Gets the error message.
    ///
    /// # Returns
    ///
    /// A human-readable message describing the error
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Converts the error to a JavaScript-friendly string.
    ///
    /// # Returns
    ///
    /// A string representation of the error suitable for JavaScript
    pub fn to_js_string(&self) -> String {
        self.message.clone()
    }
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CalculatorError {}

/// Result type for calculator operations that might fail.
///
/// This type is used for operations like division that can result in errors.
pub type CalcResult<T = f64> = Result<T, CalculatorError>;

/// Converts a string error to a CalculatorError.
///
/// This function is used to convert legacy string errors to the new error type.
///
/// # Arguments
///
/// * `error_message` - The error message
///
/// # Returns
///
/// A `CalculatorError` instance
pub fn string_to_calculator_error(error_message: &str) -> CalculatorError {
    match error_message {
        "Division by zero is not allowed" => CalculatorError::division_by_zero(),
        _ if error_message.starts_with("Invalid input") => {
            CalculatorError::invalid_input(Some(&error_message[14..]))
        }
        _ => CalculatorError::new(
            CalculatorErrorType::CalculationError,
            error_message.to_string(),
        ),
    }
}

/// Converts a CalculatorError to a JavaScript-friendly string.
///
/// # Arguments
///
/// * `error_type` - The calculator error type
/// * `message` - The error message
///
/// # Returns
///
/// A string representation of the error suitable for JavaScript
#[wasm_bindgen]
pub fn error_to_js_string_by_type(error_type: CalculatorErrorType, message: &str) -> String {
    match error_type {
        CalculatorErrorType::DivisionByZero => "Division by zero is not allowed".to_string(),
        CalculatorErrorType::InvalidInput => format!("Invalid input: {}", message),
        CalculatorErrorType::Overflow => "Result is too large to represent".to_string(),
        CalculatorErrorType::Underflow => "Result is too small to represent".to_string(),
        CalculatorErrorType::CalculationError => format!("Calculation error: {}", message),
    }
} 