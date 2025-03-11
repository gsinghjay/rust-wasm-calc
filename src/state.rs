//! Calculator state management module.
//!
//! This module provides functionality for managing the calculator's state,
//! including tracking current input, operations, and memory state.

use wasm_bindgen::prelude::*;

/// Represents the current operation being performed.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    None,
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Represents the current state of the calculator.
#[wasm_bindgen]
pub struct CalculatorState {
    /// The current display value
    display_value: String,
    /// The first operand for binary operations
    first_operand: Option<f64>,
    /// The current operation
    current_operation: Operation,
    /// Whether the display should be cleared on next input
    clear_on_next_input: bool,
    /// Whether the last button pressed was an operation
    last_pressed_operation: bool,
    /// Whether there is an error state
    error_state: bool,
}

#[wasm_bindgen]
impl CalculatorState {
    /// Creates a new calculator state with default values.
    ///
    /// # Returns
    ///
    /// A new `CalculatorState` instance with initial values.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            display_value: "0".to_string(),
            first_operand: None,
            current_operation: Operation::None,
            clear_on_next_input: false,
            last_pressed_operation: false,
            error_state: false,
        }
    }

    /// Gets the current display value.
    ///
    /// # Returns
    ///
    /// The current display value as a string.
    #[wasm_bindgen(getter)]
    pub fn display_value(&self) -> String {
        self.display_value.clone()
    }

    /// Clears the calculator state.
    ///
    /// This resets the calculator to its initial state.
    pub fn clear(&mut self) {
        self.display_value = "0".to_string();
        self.first_operand = None;
        self.current_operation = Operation::None;
        self.clear_on_next_input = false;
        self.last_pressed_operation = false;
        self.error_state = false;
    }

    /// Clears the current entry without resetting the entire calculation.
    pub fn clear_entry(&mut self) {
        self.display_value = "0".to_string();
        self.clear_on_next_input = false;
        self.error_state = false;
    }

    /// Handles input of a digit.
    ///
    /// # Arguments
    ///
    /// * `digit` - The digit to input (0-9)
    pub fn input_digit(&mut self, digit: u8) {
        if self.error_state {
            return;
        }

        if digit > 9 {
            return;
        }

        if self.clear_on_next_input {
            self.display_value = digit.to_string();
            self.clear_on_next_input = false;
        } else if self.display_value == "0" {
            self.display_value = digit.to_string();
        } else {
            self.display_value.push_str(&digit.to_string());
        }

        self.last_pressed_operation = false;
    }

    /// Handles input of a decimal point.
    pub fn input_decimal(&mut self) {
        if self.error_state {
            return;
        }

        if self.clear_on_next_input {
            self.display_value = "0.".to_string();
            self.clear_on_next_input = false;
        } else if !self.display_value.contains('.') {
            self.display_value.push('.');
        }

        self.last_pressed_operation = false;
    }

    /// Toggles the sign of the current display value.
    pub fn toggle_sign(&mut self) {
        if self.error_state {
            return;
        }

        if self.display_value != "0" {
            if self.display_value.starts_with('-') {
                self.display_value = self.display_value[1..].to_string();
            } else {
                self.display_value = format!("-{}", self.display_value);
            }
        }
    }

    /// Handles backspace/delete functionality.
    pub fn backspace(&mut self) {
        if self.error_state {
            return;
        }

        if self.clear_on_next_input {
            self.clear_entry();
            return;
        }

        if self.display_value.len() > 1 {
            self.display_value.pop();
        } else {
            self.display_value = "0".to_string();
        }
    }

    /// Sets the current operation.
    ///
    /// # Arguments
    ///
    /// * `operation` - The operation to set
    pub fn set_operation(&mut self, operation: Operation) {
        if self.error_state {
            return;
        }

        // If we already have a pending operation, calculate the result first
        if self.first_operand.is_some() && !self.last_pressed_operation {
            self.calculate();
        }

        match self.display_value.parse::<f64>() {
            Ok(value) => {
                self.first_operand = Some(value);
                self.current_operation = operation;
                self.clear_on_next_input = true;
                self.last_pressed_operation = true;
            }
            Err(_) => {
                self.error_state = true;
                self.display_value = "Error".to_string();
            }
        }
    }

    /// Calculates the result of the current operation.
    pub fn calculate(&mut self) {
        if self.error_state {
            return;
        }

        if let Some(first) = self.first_operand {
            match self.display_value.parse::<f64>() {
                Ok(second) => {
                    let result = match self.current_operation {
                        Operation::Add => Ok(first + second),
                        Operation::Subtract => Ok(first - second),
                        Operation::Multiply => Ok(first * second),
                        Operation::Divide => {
                            if second == 0.0 {
                                Err("Division by zero")
                            } else {
                                Ok(first / second)
                            }
                        }
                        Operation::None => Ok(second),
                    };

                    match result {
                        Ok(value) => {
                            // Format the result to avoid unnecessary decimal places
                            if value == (value as i64) as f64 {
                                self.display_value = format!("{}", value as i64);
                            } else {
                                self.display_value = format!("{}", value);
                            }
                            self.first_operand = Some(value);
                        }
                        Err(_) => {
                            self.error_state = true;
                            self.display_value = "Error".to_string();
                        }
                    }
                }
                Err(_) => {
                    self.error_state = true;
                    self.display_value = "Error".to_string();
                }
            }
        }

        self.current_operation = Operation::None;
        self.clear_on_next_input = true;
        self.last_pressed_operation = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_new_calculator_state() {
        let state = CalculatorState::new();
        assert_eq!(state.display_value, "0");
        assert_eq!(state.first_operand, None);
        assert_eq!(state.current_operation, Operation::None);
        assert_eq!(state.clear_on_next_input, false);
        assert_eq!(state.last_pressed_operation, false);
        assert_eq!(state.error_state, false);
    }

    #[test]
    fn test_input_digit() {
        let mut state = CalculatorState::new();
        
        // Test single digit input
        state.input_digit(5);
        assert_eq!(state.display_value, "5");
        
        // Test multiple digit input
        state.input_digit(3);
        assert_eq!(state.display_value, "53");
        
        // Test clear on next input
        state.clear_on_next_input = true;
        state.input_digit(7);
        assert_eq!(state.display_value, "7");
        assert_eq!(state.clear_on_next_input, false);
    }

    #[test]
    fn test_input_decimal() {
        let mut state = CalculatorState::new();
        
        // Test adding decimal to zero
        state.input_decimal();
        assert_eq!(state.display_value, "0.");
        
        // Test adding digits after decimal
        state.input_digit(5);
        assert_eq!(state.display_value, "0.5");
        
        // Test that second decimal is ignored
        state.input_decimal();
        assert_eq!(state.display_value, "0.5");
    }

    #[test]
    fn test_toggle_sign() {
        let mut state = CalculatorState::new();
        
        // Test toggling sign on zero (should remain zero)
        state.toggle_sign();
        assert_eq!(state.display_value, "0");
        
        // Test toggling sign on positive number
        state.input_digit(5);
        state.toggle_sign();
        assert_eq!(state.display_value, "-5");
        
        // Test toggling sign on negative number
        state.toggle_sign();
        assert_eq!(state.display_value, "5");
    }

    #[test]
    fn test_backspace() {
        let mut state = CalculatorState::new();
        
        // Test backspace on single digit (should become zero)
        state.input_digit(5);
        state.backspace();
        assert_eq!(state.display_value, "0");
        
        // Test backspace on multiple digits
        state.input_digit(1);
        state.input_digit(2);
        state.input_digit(3);
        state.backspace();
        assert_eq!(state.display_value, "12");
    }

    #[test]
    fn test_clear() {
        let mut state = CalculatorState::new();
        
        // Set up a non-default state
        state.input_digit(5);
        state.set_operation(Operation::Add);
        state.input_digit(3);
        
        // Test clear
        state.clear();
        assert_eq!(state.display_value, "0");
        assert_eq!(state.first_operand, None);
        assert_eq!(state.current_operation, Operation::None);
        assert_eq!(state.clear_on_next_input, false);
    }

    #[test]
    fn test_clear_entry() {
        let mut state = CalculatorState::new();
        
        // Set up a calculation
        state.input_digit(5);
        state.set_operation(Operation::Add);
        state.input_digit(3);
        
        // Test clear entry (should only clear current entry)
        state.clear_entry();
        assert_eq!(state.display_value, "0");
        assert_eq!(state.first_operand, Some(5.0));
        assert_eq!(state.current_operation, Operation::Add);
    }

    #[test]
    fn test_addition() {
        let mut state = CalculatorState::new();
        
        state.input_digit(5);
        state.set_operation(Operation::Add);
        state.input_digit(3);
        state.calculate();
        
        assert_eq!(state.display_value, "8");
    }

    #[test]
    fn test_subtraction() {
        let mut state = CalculatorState::new();
        
        state.input_digit(5);
        state.set_operation(Operation::Subtract);
        state.input_digit(3);
        state.calculate();
        
        assert_eq!(state.display_value, "2");
    }

    #[test]
    fn test_multiplication() {
        let mut state = CalculatorState::new();
        
        state.input_digit(5);
        state.set_operation(Operation::Multiply);
        state.input_digit(3);
        state.calculate();
        
        assert_eq!(state.display_value, "15");
    }

    #[test]
    fn test_division() {
        let mut state = CalculatorState::new();
        
        state.input_digit(6);
        state.set_operation(Operation::Divide);
        state.input_digit(3);
        state.calculate();
        
        assert_eq!(state.display_value, "2");
    }

    #[test]
    fn test_division_by_zero() {
        let mut state = CalculatorState::new();
        
        state.input_digit(5);
        state.set_operation(Operation::Divide);
        state.input_digit(0);
        state.calculate();
        
        assert_eq!(state.display_value, "Error");
        assert_eq!(state.error_state, true);
    }

    #[test]
    fn test_operation_chaining() {
        let mut state = CalculatorState::new();
        
        state.input_digit(5);
        state.set_operation(Operation::Add);
        state.input_digit(3);
        state.set_operation(Operation::Multiply);
        state.input_digit(2);
        state.calculate();
        
        assert_eq!(state.display_value, "16");
    }

    #[wasm_bindgen_test]
    fn test_calculator_state_wasm() {
        let mut state = CalculatorState::new();
        
        state.input_digit(7);
        state.input_digit(5);
        assert_eq!(state.display_value(), "75");
        
        state.set_operation(Operation::Add);
        state.input_digit(2);
        state.input_digit(5);
        state.calculate();
        
        assert_eq!(state.display_value(), "100");
    }
} 