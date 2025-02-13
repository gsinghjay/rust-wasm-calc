use wasm_bindgen_test::*;
use rust_wasm_calc::hello;

wasm_bindgen_test_configure!(run_in_browser);

// Test the hello function in a real browser environment
#[wasm_bindgen_test]
fn test_hello_in_browser() {
    let result = hello("Browser");
    assert_eq!(result, "Hello, Browser!");
}

// Test with special characters (Unicode support)
#[wasm_bindgen_test]
fn test_hello_unicode() {
    let result = hello("🌍");
    assert_eq!(result, "Hello, 🌍!");
}

// Test multiple calls (ensuring no state interference)
#[wasm_bindgen_test]
fn test_multiple_hellos() {
    assert_eq!(hello("First"), "Hello, First!");
    assert_eq!(hello("Second"), "Hello, Second!");
    assert_eq!(hello("Third"), "Hello, Third!");
} 