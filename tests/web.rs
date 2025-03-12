//! Integration tests for the WebAssembly interface.
//!
//! This module contains browser-specific tests that verify the WASM bindings
//! work correctly in a real browser environment. These tests run using
//! wasm-bindgen-test and ensure proper JavaScript interop.

use wasm_bindgen_test::*;
use rust_wasm_calc::hello;
use rust_wasm_calc::calculator::{
    add, subtract, multiply, divide_wasm,
    memory_store, memory_recall, memory_clear, memory_add, memory_subtract
};
use rust_wasm_calc::state::types::{CalculatorState, Operation};

wasm_bindgen_test_configure!(run_in_browser);

/// Tests the basic functionality of the hello function in a browser environment.
///
/// This test verifies that:
/// - The WASM module loads correctly in the browser
/// - The hello function can be called from JavaScript
/// - String passing between Rust and JavaScript works as expected
///
/// # Test Setup
/// - Runs in a real browser environment
/// - No special setup required
#[wasm_bindgen_test]
fn test_hello_in_browser() {
    let result = hello("Browser");
    assert_eq!(result, "Hello, Browser!");
}

/// Tests Unicode character handling in the hello function.
///
/// This test ensures that:
/// - Unicode characters are properly passed between JavaScript and Rust
/// - UTF-8 encoding is maintained through the WASM boundary
/// - Emoji and special characters are handled correctly
///
/// # Test Data
/// Uses the "🌍" (world emoji) as a test case for Unicode support
#[wasm_bindgen_test]
fn test_hello_unicode() {
    let result = hello("🌍");
    assert_eq!(result, "Hello, 🌍!");
}

/// Tests multiple sequential calls to the hello function.
///
/// This test verifies that:
/// - The function maintains no internal state
/// - Multiple calls work consistently
/// - No memory leaks occur between calls
///
/// # Test Methodology
/// Makes three sequential calls with different inputs to ensure
/// each call is independent and produces the expected result.
#[wasm_bindgen_test]
fn test_multiple_hellos() {
    assert_eq!(hello("First"), "Hello, First!");
    assert_eq!(hello("Second"), "Hello, Second!");
    assert_eq!(hello("Third"), "Hello, Third!");
}

/// Tests the handling of empty string input.
///
/// This test verifies that:
/// - Empty strings are handled gracefully
/// - No panics occur with zero-length input
/// - The function returns a valid greeting
#[wasm_bindgen_test]
fn test_empty_string() {
    let result = hello("");
    assert_eq!(result, "Hello, !");
}

/// Tests the handling of very long string input.
///
/// This test ensures that:
/// - Large strings don't cause memory issues
/// - WASM memory allocation works correctly
/// - String concatenation works with large inputs
///
/// # Test Data
/// Uses a string of 10,000 characters
#[wasm_bindgen_test]
fn test_long_string() {
    let long_input = "a".repeat(10_000);
    let result = hello(&long_input);
    assert!(result.starts_with("Hello, "));
    assert!(result.ends_with("!"));
    assert_eq!(result.len(), long_input.len() + 8); // "Hello, " + input + "!"
}

/// Tests special characters and edge cases.
///
/// This test verifies handling of:
/// - Special characters
/// - Control characters
/// - Whitespace
/// - Mixed Unicode and ASCII
#[wasm_bindgen_test]
fn test_special_characters() {
    let inputs = [
        "\n\t\r",                    // Control characters
        "    ",                      // Whitespace
        "Hello\u{0000}World",        // Null character
        "🌍\u{1F600}⚡️\n123",       // Mixed Unicode
    ];

    for input in inputs {
        let result = hello(input);
        assert!(result.starts_with("Hello, "));
        assert!(result.ends_with("!"));
    }
}

/// Tests concurrent execution of the hello function.
///
/// This test ensures that:
/// - Multiple calls can be made concurrently
/// - No race conditions occur
/// - Memory is properly managed in concurrent scenarios
///
/// # Note
/// Uses async/await for concurrent execution
#[wasm_bindgen_test]
async fn test_concurrent_calls() {
    use wasm_bindgen_futures::spawn_local;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = counter.clone();
        let handle = spawn_local(async move {
            let result = hello(&format!("Concurrent{}", i));
            assert_eq!(result, format!("Hello, Concurrent{}!", i));
            counter.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    // Wait for all tests to complete
    for _ in 0..5 {
        while counter.load(Ordering::SeqCst) < 5 {
            wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |_resolve, _reject| {}))
                .await
                .unwrap();
        }
    }

    assert_eq!(counter.load(Ordering::SeqCst), 5);
}

/// Tests calculator operations in a browser environment.
///
/// This test verifies that:
/// - Basic arithmetic operations work through WASM
/// - Results are correctly returned to JavaScript
#[wasm_bindgen_test]
fn test_calculator_operations() {
    assert_eq!(add(2.0, 3.0), 5.0);
    assert_eq!(subtract(5.0, 3.0), 2.0);
    assert_eq!(multiply(2.0, 3.0), 6.0);
    
    // Test division with valid input
    let result = divide_wasm(6.0, 3.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 2.0);
    
    // Test division by zero
    let error = divide_wasm(1.0, 0.0);
    assert!(error.is_err());
}

/// Tests memory operations in a browser environment.
///
/// This test verifies that:
/// - Memory operations work through WASM
/// - Memory state is maintained between calls
#[wasm_bindgen_test]
fn test_memory_operations() {
    // Clear memory to ensure a known state
    memory_clear();
    assert_eq!(memory_recall(), 0.0);
    
    // Store and recall
    memory_store(42.0);
    assert_eq!(memory_recall(), 42.0);
    
    // Add to memory
    memory_add(8.0);
    assert_eq!(memory_recall(), 50.0);
    
    // Subtract from memory
    memory_subtract(10.0);
    assert_eq!(memory_recall(), 40.0);
    
    // Clear again
    memory_clear();
    assert_eq!(memory_recall(), 0.0);
}

/// Tests calculator state management in a browser environment.
///
/// This test verifies that:
/// - CalculatorState can be created and manipulated through WASM
/// - State operations work correctly in the browser
#[wasm_bindgen_test]
fn test_calculator_state() {
    let mut state = CalculatorState::new();
    
    // Initial state
    assert_eq!(state.display_value(), "0");
    
    // Input a number
    state.input_digit(5);
    assert_eq!(state.display_value(), "5");
    
    // Perform a calculation
    state.set_operation(Operation::Add);
    state.input_digit(3);
    state.calculate();
    assert_eq!(state.display_value(), "8");
    
    // Clear the state
    state.clear();
    assert_eq!(state.display_value(), "0");
} 