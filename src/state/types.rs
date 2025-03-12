//! Type definitions for calculator state management.
//!
//! This module defines the core types used for managing calculator state:
//! - Operation enum for representing arithmetic operations
//! - CalculatorState struct for tracking the calculator's current state

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
    pub(crate) display_value: String,
    /// The first operand for binary operations
    pub(crate) first_operand: Option<f64>,
    /// The current operation
    pub(crate) current_operation: Operation,
    /// Whether the display should be cleared on next input
    pub(crate) clear_on_next_input: bool,
    /// Whether the last button pressed was an operation
    pub(crate) last_pressed_operation: bool,
    /// Whether there is an error state
    pub(crate) error_state: bool,
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self::new()
    }
}

/// Constructor and basic getters for CalculatorState
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
    #[wasm_bindgen]
    pub fn display_value(&self) -> String {
        self.display_value.clone()
    }
} 