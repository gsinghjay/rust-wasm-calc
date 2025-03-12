//! Tests for calculator memory operations.
//!
//! This module contains unit tests for the memory-related operations:
//! - Store a value in memory
//! - Recall the current memory value
//! - Clear the memory
//! - Add to memory
//! - Subtract from memory

use rust_wasm_calc::calculator::{
    memory_store, memory_recall, memory_clear, memory_add, memory_subtract
};

/// Tests for basic memory operations (store, recall, clear).
#[cfg(test)]
mod basic_memory_tests {
    use super::*;

    #[test]
    fn test_memory_store_and_recall() {
        // Store a value and recall it
        memory_store(42.0);
        assert_eq!(memory_recall(), 42.0);

        // Store a different value and recall it
        memory_store(123.456);
        assert_eq!(memory_recall(), 123.456);

        // Store a negative value and recall it
        memory_store(-789.0);
        assert_eq!(memory_recall(), -789.0);
    }

    #[test]
    fn test_memory_clear() {
        // Store a value
        memory_store(42.0);
        assert_eq!(memory_recall(), 42.0);

        // Clear memory and verify it's zero
        memory_clear();
        assert_eq!(memory_recall(), 0.0);

        // Store another value
        memory_store(99.9);
        assert_eq!(memory_recall(), 99.9);

        // Clear again
        memory_clear();
        assert_eq!(memory_recall(), 0.0);
    }

    #[test]
    fn test_memory_initial_value() {
        // Clear memory to ensure a known state
        memory_clear();
        
        // Memory should be initialized to 0.0
        assert_eq!(memory_recall(), 0.0);
    }
}

/// Tests for memory arithmetic operations (add, subtract).
#[cfg(test)]
mod memory_arithmetic_tests {
    use super::*;

    #[test]
    fn test_memory_add() {
        // Clear memory to ensure a known state
        memory_clear();
        assert_eq!(memory_recall(), 0.0);

        // Add a value to memory
        memory_add(5.0);
        assert_eq!(memory_recall(), 5.0);

        // Add another value
        memory_add(3.0);
        assert_eq!(memory_recall(), 8.0);

        // Add a negative value
        memory_add(-2.0);
        assert_eq!(memory_recall(), 6.0);
    }

    #[test]
    fn test_memory_subtract() {
        // Clear memory and set initial value
        memory_clear();
        memory_store(10.0);
        assert_eq!(memory_recall(), 10.0);

        // Subtract a value
        memory_subtract(3.0);
        assert_eq!(memory_recall(), 7.0);

        // Subtract another value
        memory_subtract(2.0);
        assert_eq!(memory_recall(), 5.0);

        // Subtract a negative value (which adds)
        memory_subtract(-5.0);
        assert_eq!(memory_recall(), 10.0);
    }

    #[test]
    fn test_memory_combined_operations() {
        // Clear memory to ensure a known state
        memory_clear();
        
        // Perform a series of operations
        memory_add(10.0);
        memory_subtract(5.0);
        memory_add(7.0);
        memory_subtract(2.0);
        
        // Verify final result
        assert_eq!(memory_recall(), 10.0);
        
        // Clear and verify
        memory_clear();
        assert_eq!(memory_recall(), 0.0);
    }
}

/// Tests for edge cases in memory operations.
#[cfg(test)]
mod memory_edge_cases {
    use super::*;

    #[test]
    fn test_memory_floating_point() {
        memory_clear();
        
        // Test with floating point values
        memory_store(0.1);
        memory_add(0.2);
        
        // Due to floating point precision, this won't be exactly 0.3
        assert_eq!(memory_recall(), 0.30000000000000004);
    }

    #[test]
    fn test_memory_large_values() {
        memory_clear();
        
        // Test with very large values
        let large_value = 1.0e100;
        memory_store(large_value);
        assert_eq!(memory_recall(), large_value);
        
        // Add another large value
        memory_add(large_value);
        assert_eq!(memory_recall(), 2.0 * large_value);
    }

    #[test]
    fn test_memory_small_values() {
        memory_clear();
        
        // Test with very small values
        let small_value = 1.0e-100;
        memory_store(small_value);
        assert_eq!(memory_recall(), small_value);
        
        // Add another small value
        memory_add(small_value);
        assert_eq!(memory_recall(), 2.0 * small_value);
    }
} 