//! Operations for manipulating calculator state.
//!
//! This module provides the implementation of operations that can be performed
//! on the calculator state, such as:
//! - Clearing the calculator
//! - Inputting digits and decimal points
//! - Setting operations
//! - Calculating results

use wasm_bindgen::prelude::*;
use super::types::{CalculatorState, Operation};
use crate::errors::CalculatorError;

#[wasm_bindgen]
impl CalculatorState {
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
                                Err(CalculatorError::division_by_zero())
                            } else {
                                Ok(first / second)
                            }
                        }
                        Operation::None => Ok(second),
                    };

                    match result {
                        Ok(value) => {
                            // Check for overflow/underflow
                            if value.is_infinite() {
                                self.error_state = true;
                                self.display_value = "Error: Overflow".to_string();
                            } else if value.is_nan() {
                                self.error_state = true;
                                self.display_value = "Error: Invalid operation".to_string();
                            } else {
                                // Format the result to avoid unnecessary decimal places
                                if value == (value as i64) as f64 {
                                    self.display_value = format!("{}", value as i64);
                                } else {
                                    self.display_value = format!("{}", value);
                                }
                                self.first_operand = Some(value);
                            }
                        }
                        Err(error) => {
                            self.error_state = true;
                            self.display_value = format!("Error: {}", error.message());
                        }
                    }
                }
                Err(_) => {
                    self.error_state = true;
                    self.display_value = "Error: Invalid input".to_string();
                }
            }
        }

        self.current_operation = Operation::None;
        self.clear_on_next_input = true;
        self.last_pressed_operation = false;
    }
} 