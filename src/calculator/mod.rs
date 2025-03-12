//! Calculator module for the Rust WASM Calculator.
//!
//! This module provides the core calculator functionality including:
//! - Basic arithmetic operations (add, subtract, multiply, divide)
//! - Memory operations (store, recall, clear, add, subtract)
//!
//! The module is designed to be used with WebAssembly through wasm-bindgen.

// Export submodules
pub mod operations;
pub mod memory;

// Re-export commonly used types and functions
pub use operations::{add, subtract, multiply, divide, divide_wasm};
pub use memory::{memory_store, memory_recall, memory_clear, memory_add, memory_subtract};
pub use crate::errors::CalcResult; 