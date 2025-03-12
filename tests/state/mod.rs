//! Tests for calculator state management.
//!
//! This module contains tests for the calculator state management functionality:
//! - State initialization
//! - State operations
//! - State transitions

// Re-export test modules
pub mod types_tests;
pub mod operations_tests;

// Integration tests that combine multiple state operations
#[cfg(test)]
mod integration_tests {
    use rust_wasm_calc::state::types::CalculatorState;
    use rust_wasm_calc::state::types::Operation;

    #[test]
    fn test_full_calculation_sequence() {
        let mut state = CalculatorState::new();
        
        // Initial state check
        assert_eq!(state.display_value(), "0");
        
        // Input first number: 5
        state.input_digit(5);
        assert_eq!(state.display_value(), "5");
        
        // Set addition operation
        state.set_operation(Operation::Add);
        
        // Input second number: 3
        state.input_digit(3);
        assert_eq!(state.display_value(), "3");
        
        // Calculate result
        state.calculate();
        assert_eq!(state.display_value(), "8");
        
        // Continue calculation with multiplication
        state.set_operation(Operation::Multiply);
        state.input_digit(2);
        state.calculate();
        
        // Final result should be 8 * 2 = 16
        assert_eq!(state.display_value(), "16");
    }

    #[test]
    fn test_error_handling_and_recovery() {
        let mut state = CalculatorState::new();
        
        // Set up division by zero
        state.input_digit(5);
        state.set_operation(Operation::Divide);
        state.input_digit(0);
        
        // Calculate should result in error
        state.calculate();
        assert!(state.display_value().contains("Error"));
        
        // Clear should recover from error state
        state.clear();
        assert_eq!(state.display_value(), "0");
        
        // Should be able to perform calculations again
        state.input_digit(7);
        state.set_operation(Operation::Add);
        state.input_digit(3);
        state.calculate();
        assert_eq!(state.display_value(), "10");
    }
} 