/**
 * Calculator Memory Module
 * 
 * This module handles memory operations for the calculator.
 * It provides a clean interface to the WASM memory functions.
 */

/**
 * Initializes the memory module with WASM functions
 * 
 * @param {Object} wasmFunctions - Object containing WASM memory functions
 * @returns {Object} Memory operations interface
 */
export function initMemory(wasmFunctions) {
  const { memory_store, memory_recall, memory_clear, memory_add, memory_subtract } = wasmFunctions;
  
  return {
    /**
     * Stores a value in memory
     * 
     * @param {number} value - The value to store
     */
    store(value) {
      memory_store(value);
    },
    
    /**
     * Recalls the current value from memory
     * 
     * @returns {number} The current memory value
     */
    recall() {
      return memory_recall();
    },
    
    /**
     * Clears the memory
     */
    clear() {
      memory_clear();
    },
    
    /**
     * Adds a value to memory
     * 
     * @param {number} value - The value to add
     */
    add(value) {
      memory_add(value);
    },
    
    /**
     * Subtracts a value from memory
     * 
     * @param {number} value - The value to subtract
     */
    subtract(value) {
      memory_subtract(value);
    }
  };
} 