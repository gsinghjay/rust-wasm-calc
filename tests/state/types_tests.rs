//! Tests for calculator state type definitions.
//!
//! This module contains tests for the state type definitions:
//! - CalculatorState struct
//! - Operation enum

use rust_wasm_calc::state::types::{CalculatorState, Operation};

/// Tests for the CalculatorState struct initialization and basic properties.
#[cfg(test)]
mod calculator_state_tests {
    use super::*;

    #[test]
    fn test_calculator_state_new() {
        let state = CalculatorState::new();
        
        // Check initial values
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
    fn test_display_value_getter() {
        let state = CalculatorState::new();
        assert_eq!(state.display_value(), "0");
    }
}

/// Tests for the Operation enum.
#[cfg(test)]
mod operation_enum_tests {
    use super::*;

    #[test]
    fn test_operation_equality() {
        assert_eq!(Operation::None, Operation::None);
        assert_eq!(Operation::Add, Operation::Add);
        assert_eq!(Operation::Subtract, Operation::Subtract);
        assert_eq!(Operation::Multiply, Operation::Multiply);
        assert_eq!(Operation::Divide, Operation::Divide);
        
        assert_ne!(Operation::None, Operation::Add);
        assert_ne!(Operation::Add, Operation::Subtract);
        assert_ne!(Operation::Subtract, Operation::Multiply);
        assert_ne!(Operation::Multiply, Operation::Divide);
        assert_ne!(Operation::Divide, Operation::None);
    }
    
    #[test]
    fn test_operation_copy() {
        let op1 = Operation::Add;
        let op2 = op1;
        
        // Both should be valid after copy
        assert_eq!(op1, Operation::Add);
        assert_eq!(op2, Operation::Add);
    }
    
    #[test]
    fn test_operation_debug() {
        assert_eq!(format!("{:?}", Operation::None), "None");
        assert_eq!(format!("{:?}", Operation::Add), "Add");
        assert_eq!(format!("{:?}", Operation::Subtract), "Subtract");
        assert_eq!(format!("{:?}", Operation::Multiply), "Multiply");
        assert_eq!(format!("{:?}", Operation::Divide), "Divide");
    }
} 