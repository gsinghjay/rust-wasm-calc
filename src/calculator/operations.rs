//! Calculator operations module.
//!
//! This module provides the core arithmetic operations for the calculator:
//! - Addition
//! - Subtraction
//! - Multiplication
//! - Division
//!
//! All operations are exposed to JavaScript through WebAssembly bindings.

use wasm_bindgen::prelude::*;
use crate::errors::{CalculatorError, CalcResult};

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
/// * `Err(error)` - A calculator error if division by zero is attempted
///
/// # Examples
///
/// ```
/// use rust_wasm_calc::calculator::divide;
///
/// let result = divide(6.0, 3.0);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), 2.0);
///
/// let error = divide(1.0, 0.0);
/// assert!(error.is_err());
/// ```
pub fn divide(a: f64, b: f64) -> CalcResult {
    if b == 0.0 {
        Err(CalculatorError::division_by_zero())
    } else {
        Ok(a / b)
    }
}

/// Divides the first number by the second and returns the result.
/// This is a WebAssembly-friendly version that returns a Result as a JsValue.
///
/// # Arguments
///
/// * `a` - Dividend (numerator)
/// * `b` - Divisor (denominator)
///
/// # Returns
///
/// The quotient `a / b` if successful, or an error message if division by zero is attempted
#[wasm_bindgen(js_name = divide)]
pub fn divide_wasm(a: f64, b: f64) -> Result<f64, JsValue> {
    divide(a, b).map_err(|e| JsValue::from_str(e.message()))
} 