use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Regular Rust unit test
    #[test]
    fn test_hello_rust() {
        let result = hello("Rust");
        assert_eq!(result, "Hello, Rust!");
    }

    // WASM-specific test that runs in the browser
    #[wasm_bindgen_test]
    fn test_hello_wasm_browser() {
        let result = hello("WASM");
        assert_eq!(result, "Hello, WASM!");
    }

    // WASM-specific test that runs in Node.js
    #[wasm_bindgen_test]
    fn test_hello_empty_string() {
        let result = hello("");
        assert_eq!(result, "Hello, !");
    }

    // Example of testing error cases (for future calculator functions)
    #[wasm_bindgen_test]
    fn test_hello_long_string() {
        let long_name = "a".repeat(1000);
        let result = hello(&long_name);
        assert!(result.starts_with("Hello, "));
        assert!(result.ends_with("!"));
    }
}
