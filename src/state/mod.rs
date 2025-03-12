//! Calculator state management module.
//!
//! This module provides functionality for managing the calculator's state,
//! including tracking current input, operations, and memory state.
//!
//! The module is organized into:
//! - Type definitions (Operation enum, CalculatorState struct)
//! - State operations (methods for manipulating calculator state)

// Export submodules
pub mod types;
pub mod operations;

// Re-export commonly used types and functions
pub use types::{Operation, CalculatorState}; 