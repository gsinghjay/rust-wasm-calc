//! Tests for calculator operations.
//!
//! This module contains unit tests for the basic arithmetic operations:
//! - Addition
//! - Subtraction
//! - Multiplication
//! - Division

use rust_wasm_calc::calculator::{add, subtract, multiply, divide};

/// Tests for the add function.
#[cfg(test)]
mod add_tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(add(0.0, 5.0), 5.0);
        assert_eq!(add(100.0, 50.0), 150.0);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-2.0, -3.0), -5.0);
        assert_eq!(add(-5.0, 0.0), -5.0);
        assert_eq!(add(-10.0, -20.0), -30.0);
    }

    #[test]
    fn test_add_mixed_signs() {
        assert_eq!(add(5.0, -3.0), 2.0);
        assert_eq!(add(-5.0, 8.0), 3.0);
        assert_eq!(add(-10.0, 10.0), 0.0);
    }

    #[test]
    fn test_add_floating_point() {
        assert_eq!(add(0.1, 0.2), 0.30000000000000004); // Floating point precision
        assert_eq!(add(1.5, 2.5), 4.0);
        assert_eq!(add(0.5, -0.5), 0.0);
    }
}

/// Tests for the subtract function.
#[cfg(test)]
mod subtract_tests {
    use super::*;

    #[test]
    fn test_subtract_positive_numbers() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
        assert_eq!(subtract(10.0, 5.0), 5.0);
        assert_eq!(subtract(5.0, 0.0), 5.0);
    }

    #[test]
    fn test_subtract_negative_numbers() {
        assert_eq!(subtract(-5.0, -3.0), -2.0);
        assert_eq!(subtract(-10.0, -15.0), 5.0);
        assert_eq!(subtract(-5.0, 0.0), -5.0);
    }

    #[test]
    fn test_subtract_mixed_signs() {
        assert_eq!(subtract(5.0, -3.0), 8.0);
        assert_eq!(subtract(-5.0, 3.0), -8.0);
        assert_eq!(subtract(0.0, -5.0), 5.0);
    }

    #[test]
    fn test_subtract_floating_point() {
        assert_eq!(subtract(0.3, 0.1), 0.19999999999999998); // Floating point precision
        assert_eq!(subtract(10.5, 5.5), 5.0);
        assert_eq!(subtract(1.0, 1.0), 0.0);
    }
}

/// Tests for the multiply function.
#[cfg(test)]
mod multiply_tests {
    use super::*;

    #[test]
    fn test_multiply_positive_numbers() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
        assert_eq!(multiply(0.0, 5.0), 0.0);
        assert_eq!(multiply(10.0, 10.0), 100.0);
    }

    #[test]
    fn test_multiply_negative_numbers() {
        assert_eq!(multiply(-2.0, -3.0), 6.0);
        assert_eq!(multiply(-5.0, 0.0), 0.0);
        assert_eq!(multiply(-10.0, -10.0), 100.0);
    }

    #[test]
    fn test_multiply_mixed_signs() {
        assert_eq!(multiply(5.0, -3.0), -15.0);
        assert_eq!(multiply(-5.0, 3.0), -15.0);
        assert_eq!(multiply(0.0, -5.0), 0.0);
    }

    #[test]
    fn test_multiply_floating_point() {
        assert_eq!(multiply(0.1, 0.2), 0.020000000000000004); // Floating point precision
        assert_eq!(multiply(1.5, 2.0), 3.0);
        assert_eq!(multiply(0.5, -2.0), -1.0);
    }
}

/// Tests for the divide function.
#[cfg(test)]
mod divide_tests {
    use super::*;

    #[test]
    fn test_divide_positive_numbers() {
        let result = divide(6.0, 3.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.0);

        let result = divide(10.0, 2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_divide_negative_numbers() {
        let result = divide(-6.0, -3.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.0);

        let result = divide(-10.0, -2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_divide_mixed_signs() {
        let result = divide(6.0, -3.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), -2.0);

        let result = divide(-10.0, 2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), -5.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(5.0, 0.0);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.message(), "Division by zero");
    }

    #[test]
    fn test_divide_floating_point() {
        let result = divide(1.0, 3.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1.0 / 3.0);

        let result = divide(5.5, 2.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.75);
    }
} 