//! Calculator memory operations module.
//!
//! This module provides memory-related operations for the calculator:
//! - Store a value in memory
//! - Recall the current memory value
//! - Clear the memory
//! - Add to memory
//! - Subtract from memory
//!
//! All operations are exposed to JavaScript through WebAssembly bindings.

use wasm_bindgen::prelude::*;

// Memory storage for calculator operations
thread_local! {
    static MEMORY: std::cell::RefCell<f64> = std::cell::RefCell::new(0.0);
}

/// Stores a value in the calculator's memory.
///
/// This function overwrites any previous value stored in memory.
///
/// # Arguments
///
/// * `value` - The value to store in memory
///
/// # Examples
///
/// ```
/// use rust_wasm_calc::calculator::{memory_store, memory_recall};
///
/// memory_store(42.0);
/// assert_eq!(memory_recall(), 42.0);
/// ```
#[wasm_bindgen]
pub fn memory_store(value: f64) {
    MEMORY.with(|memory| {
        *memory.borrow_mut() = value;
    });
}

/// Recalls the current value stored in the calculator's memory.
///
/// # Returns
///
/// The current value stored in memory
///
/// # Examples
///
/// ```
/// use rust_wasm_calc::calculator::{memory_store, memory_recall};
///
/// memory_store(42.0);
/// assert_eq!(memory_recall(), 42.0);
/// ```
#[wasm_bindgen]
pub fn memory_recall() -> f64 {
    MEMORY.with(|memory| {
        *memory.borrow()
    })
}

/// Clears the calculator's memory by setting it to zero.
///
/// # Examples
///
/// ```
/// use rust_wasm_calc::calculator::{memory_store, memory_clear, memory_recall};
///
/// memory_store(42.0);
/// memory_clear();
/// assert_eq!(memory_recall(), 0.0);
/// ```
#[wasm_bindgen]
pub fn memory_clear() {
    MEMORY.with(|memory| {
        *memory.borrow_mut() = 0.0;
    });
}

/// Adds a value to the current value stored in memory.
///
/// # Arguments
///
/// * `value` - The value to add to the memory
///
/// # Examples
///
/// ```
/// use rust_wasm_calc::calculator::{memory_clear, memory_add, memory_recall};
///
/// memory_clear();
/// memory_add(5.0);
/// memory_add(3.0);
/// assert_eq!(memory_recall(), 8.0);
/// ```
#[wasm_bindgen]
pub fn memory_add(value: f64) {
    MEMORY.with(|memory| {
        *memory.borrow_mut() += value;
    });
}

/// Subtracts a value from the current value stored in memory.
///
/// # Arguments
///
/// * `value` - The value to subtract from the memory
///
/// # Examples
///
/// ```
/// use rust_wasm_calc::calculator::{memory_store, memory_subtract, memory_recall};
///
/// memory_store(10.0);
/// memory_subtract(3.0);
/// assert_eq!(memory_recall(), 7.0);
/// ```
#[wasm_bindgen]
pub fn memory_subtract(value: f64) {
    MEMORY.with(|memory| {
        *memory.borrow_mut() -= value;
    });
} 