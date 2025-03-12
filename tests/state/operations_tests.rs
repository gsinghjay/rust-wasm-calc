//! Tests for calculator state operations.
//!
//! This module contains tests for the state operations:
//! - Clearing the calculator
//! - Inputting digits and decimal points
//! - Setting operations
//! - Calculating results
//! - Handling errors

use rust_wasm_calc::state::types::{CalculatorState, Operation};

/// Tests for basic state operations (clear, clear_entry).
#[cfg(test)]
mod basic_operations_tests {
    use super::*;

    #[test]
    fn test_clear() {
        let mut state = CalculatorState::new();
        
        // Set up some state
        state.input_digit(5);
        state.set_operation(Operation::Add);
        state.input_digit(3);
        
        // Clear should reset everything
        state.clear();
        
        // Check that state is reset
        assert_eq!(state.display_value(), "0");
        
        // Internal state checks (using debug representation)
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("display_value: \"0\""));
        assert!(debug_str.contains("first_operand: None"));
        assert!(debug_str.contains("current_operation: None"));
        assert!(debug_str.contains("clear_on_next_input: false"));
        assert!(debug_str.contains("last_pressed_operation: false"));
        assert!(debug_str.contains("error_state: false"));
    }

    #[test]
    fn test_clear_entry() {
        let mut state = CalculatorState::new();
        
        // Set up some state
        state.input_digit(5);
        state.set_operation(Operation::Add);
        state.input_digit(3);
        
        // Clear entry should only reset the current entry
        state.clear_entry();
        
        // Display should be reset to 0
        assert_eq!(state.display_value(), "0");
        
        // But operation should still be set
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("current_operation: Add"));
    }
}

/// Tests for digit input operations.
#[cfg(test)]
mod input_operations_tests {
    use super::*;

    #[test]
    fn test_input_digit() {
        let mut state = CalculatorState::new();
        
        // Input a single digit
        state.input_digit(5);
        assert_eq!(state.display_value(), "5");
        
        // Input another digit (should append)
        state.input_digit(3);
        assert_eq!(state.display_value(), "53");
        
        // Input more digits
        state.input_digit(0);
        state.input_digit(1);
        assert_eq!(state.display_value(), "5301");
    }

    #[test]
    fn test_input_digit_after_operation() {
        let mut state = CalculatorState::new();
        
        // Input first number
        state.input_digit(5);
        assert_eq!(state.display_value(), "5");
        
        // Set operation
        state.set_operation(Operation::Add);
        
        // Input second number (should start fresh)
        state.input_digit(3);
        assert_eq!(state.display_value(), "3");
    }

    #[test]
    fn test_input_decimal() {
        let mut state = CalculatorState::new();
        
        // Input a decimal point
        state.input_decimal();
        assert_eq!(state.display_value(), "0.");
        
        // Input digits after decimal
        state.input_digit(5);
        assert_eq!(state.display_value(), "0.5");
        
        // Input more digits
        state.input_digit(2);
        state.input_digit(3);
        assert_eq!(state.display_value(), "0.523");
        
        // Second decimal should be ignored
        state.input_decimal();
        assert_eq!(state.display_value(), "0.523");
    }

    #[test]
    fn test_toggle_sign() {
        let mut state = CalculatorState::new();
        
        // Input a number
        state.input_digit(5);
        assert_eq!(state.display_value(), "5");
        
        // Toggle sign
        state.toggle_sign();
        assert_eq!(state.display_value(), "-5");
        
        // Toggle sign again
        state.toggle_sign();
        assert_eq!(state.display_value(), "5");
        
        // Test with decimal
        state.clear();
        state.input_digit(1);
        state.input_decimal();
        state.input_digit(5);
        assert_eq!(state.display_value(), "1.5");
        
        state.toggle_sign();
        assert_eq!(state.display_value(), "-1.5");
    }

    #[test]
    fn test_backspace() {
        let mut state = CalculatorState::new();
        
        // Input a number
        state.input_digit(5);
        state.input_digit(4);
        state.input_digit(3);
        assert_eq!(state.display_value(), "543");
        
        // Backspace
        state.backspace();
        assert_eq!(state.display_value(), "54");
        
        // Backspace again
        state.backspace();
        assert_eq!(state.display_value(), "5");
        
        // Backspace to empty (should show 0)
        state.backspace();
        assert_eq!(state.display_value(), "0");
        
        // Backspace on 0 should still be 0
        state.backspace();
        assert_eq!(state.display_value(), "0");
    }
}

/// Tests for arithmetic operations.
#[cfg(test)]
mod arithmetic_operations_tests {
    use super::*;

    #[test]
    fn test_set_operation() {
        let mut state = CalculatorState::new();
        
        // Input a number
        state.input_digit(5);
        
        // Set operation
        state.set_operation(Operation::Add);
        
        // Internal state should have the operation set
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("current_operation: Add"));
        assert!(debug_str.contains("first_operand: Some(5"));
    }

    #[test]
    fn test_addition() {
        let mut state = CalculatorState::new();
        
        // 5 + 3 = 8
        state.input_digit(5);
        state.set_operation(Operation::Add);
        state.input_digit(3);
        state.calculate();
        
        assert_eq!(state.display_value(), "8");
    }

    #[test]
    fn test_subtraction() {
        let mut state = CalculatorState::new();
        
        // 10 - 4 = 6
        state.input_digit(1);
        state.input_digit(0);
        state.set_operation(Operation::Subtract);
        state.input_digit(4);
        state.calculate();
        
        assert_eq!(state.display_value(), "6");
    }

    #[test]
    fn test_multiplication() {
        let mut state = CalculatorState::new();
        
        // 6 * 7 = 42
        state.input_digit(6);
        state.set_operation(Operation::Multiply);
        state.input_digit(7);
        state.calculate();
        
        assert_eq!(state.display_value(), "42");
    }

    #[test]
    fn test_division() {
        let mut state = CalculatorState::new();
        
        // 10 / 2 = 5
        state.input_digit(1);
        state.input_digit(0);
        state.set_operation(Operation::Divide);
        state.input_digit(2);
        state.calculate();
        
        assert_eq!(state.display_value(), "5");
    }

    #[test]
    fn test_division_by_zero() {
        let mut state = CalculatorState::new();
        
        // 5 / 0 = Error
        state.input_digit(5);
        state.set_operation(Operation::Divide);
        state.input_digit(0);
        state.calculate();
        
        // Should show error
        assert!(state.display_value().contains("Error"));
        
        // Error state should be set
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("error_state: true"));
    }

    #[test]
    fn test_chained_operations() {
        let mut state = CalculatorState::new();
        
        // 5 + 3 = 8
        state.input_digit(5);
        state.set_operation(Operation::Add);
        state.input_digit(3);
        state.calculate();
        assert_eq!(state.display_value(), "8");
        
        // 8 * 2 = 16
        state.set_operation(Operation::Multiply);
        state.input_digit(2);
        state.calculate();
        assert_eq!(state.display_value(), "16");
        
        // 16 - 6 = 10
        state.set_operation(Operation::Subtract);
        state.input_digit(6);
        state.calculate();
        assert_eq!(state.display_value(), "10");
        
        // 10 / 5 = 2
        state.set_operation(Operation::Divide);
        state.input_digit(5);
        state.calculate();
        assert_eq!(state.display_value(), "2");
    }
} 