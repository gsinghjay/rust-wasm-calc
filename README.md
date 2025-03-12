# Rust WASM Calculator

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![wasm-pack](https://img.shields.io/badge/wasm--pack-latest-blue.svg)](https://rustwasm.github.io/wasm-pack/)
[![Bootstrap](https://img.shields.io/badge/bootstrap-5.3.0-purple.svg)](https://getbootstrap.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Rust WASM Calculator is a WebAssembly calculator application. It is built using Rust and Bootstrap 5. The project follows test-driven development practices and SOLID principles.

## 🚀 Quick Start

1. **Prerequisites:** Ensure you have Rust, wasm-pack, and Node.js installed.
2. **Clone Repository:** Clone the repository to your local machine.
   ```bash
   git clone https://github.com/your-username/rust-wasm-calc.git # Replace with actual repo URL
   cd rust-wasm-calc
   ```
3. **Build WASM:** Build the WebAssembly module.
   ```bash
   wasm-pack build --target web
   ```
4. **Start Development Server:** Run a local HTTP server to serve the application.
   ```bash
   npm install -g http-server # If not already installed
   http-server . -p 8080
   ```
5. **Access in Browser:** Open your browser and go to `http://localhost:8080`.

## 📦 Core Package: `rust-wasm-calc`

### Installation

The `rust-wasm-calc` package is built to WebAssembly. Use `wasm-pack build --target web` to generate the package in the `pkg/` directory.

### Project Structure

The codebase is organized into the following modules:

- **calculator**: Core calculator functionality
  - **operations.rs**: Basic arithmetic operations
  - **memory.rs**: Memory-related functions
- **state**: State management for the calculator
  - **types.rs**: Type definitions (Operation enum, CalculatorState struct)
  - **operations.rs**: Methods for manipulating calculator state
- **errors.rs**: Error handling and standardized error types

### API

#### `hello(name: &str) -> String`

- **Description:**  Returns a greeting string.
- **Parameters:**
    - `name`:  The name to include in the greeting.
- **Returns:**  A string formatted as "Hello, {name}!".

#### Calculator Operations

##### `add(a: f64, b: f64) -> f64`

- **Description:** Adds two numbers and returns the result.
- **Parameters:**
    - `a`: First operand
    - `b`: Second operand
- **Returns:** The sum of `a` and `b`

##### `subtract(a: f64, b: f64) -> f64`

- **Description:** Subtracts the second number from the first and returns the result.
- **Parameters:**
    - `a`: First operand
    - `b`: Second operand
- **Returns:** The difference `a - b`

##### `multiply(a: f64, b: f64) -> f64`

- **Description:** Multiplies two numbers and returns the result.
- **Parameters:**
    - `a`: First operand
    - `b`: Second operand
- **Returns:** The product of `a` and `b`

##### `divide(a: f64, b: f64) -> Result<f64, CalculatorError>`

- **Description:** Divides the first number by the second and returns the result.
- **Parameters:**
    - `a`: Dividend (numerator)
    - `b`: Divisor (denominator)
- **Returns:** 
    - `Ok(result)`: The quotient `a / b` if successful
    - `Err(error)`: A `CalculatorError` if division by zero is attempted

#### Memory Functions

- **`memory_store(value: f64)`**: Stores a value in the calculator's memory
- **`memory_recall() -> f64`**: Recalls the current value stored in memory
- **`memory_clear()`**: Clears the calculator's memory by setting it to zero
- **`memory_add(value: f64)`**: Adds a value to the current value stored in memory
- **`memory_subtract(value: f64)`**: Subtracts a value from the current value stored in memory

#### Error Handling

The calculator uses a standardized error handling system:

- **`CalculatorErrorType`**: Enum representing different error types:
  - `DivisionByZero`: Division by zero error
  - `InvalidInput`: Invalid input error
  - `Overflow`: Result too large
  - `Underflow`: Result too small
  - `CalculationError`: General calculation error

- **`CalculatorError`**: Struct containing error type and message

- **`error_to_js_string_by_type(error_type, message)`**: Converts error information to a JavaScript-friendly string

**Example Usage (JavaScript):**

```javascript
import init, { hello, add, subtract, multiply, divide, memory_store, memory_recall } from './pkg/rust_wasm_calc.js';

async function run() {
  await init();
  
  // Basic greeting
  const message = hello("User");
  console.log(message); // Output: Hello, User!
  
  // Calculator operations
  console.log(add(2, 3)); // Output: 5
  console.log(subtract(5, 3)); // Output: 2
  console.log(multiply(2, 3)); // Output: 6
  
  // Division with error handling
  try {
    const result = divide(6, 3);
    console.log(result); // Output: 2
  } catch (e) {
    console.error(e); // Error message if division by zero
  }
  
  // Memory operations
  memory_store(10);
  memory_add(5);
  console.log(memory_recall()); // Output: 15
  memory_subtract(3);
  console.log(memory_recall()); // Output: 12
  memory_clear();
  console.log(memory_recall()); // Output: 0
}

run();
```

## ⚙️ Configuration

### Dependencies

- **Rust:** Programming language for core logic.
- **wasm-pack:** Tool to build Rust to WebAssembly.
- **Node.js:**  Required for development server and potentially testing.
- **wasm-bindgen:**  Facilitates communication between Rust and JavaScript.
- **Bootstrap 5.3.0:**  CSS framework for user interface.
- **Bootstrap Icons:** Icon library for UI elements.
- **console_error_panic_hook:**  Provides improved error messages in the browser console.

### Features

- `default`: Enables the `console_error_panic_hook` feature. This is recommended for better debugging in browser environments.

## 🧪 Advanced Usage

### Running Tests

- **Unit Tests (Rust):** Execute Rust unit tests.
  ```bash
  cargo test
  ```
- **WASM Tests (Node.js):** Run WASM tests in a Node.js environment.
  ```bash
  wasm-pack test --node
  ```
- **WASM Tests (Browser):** Run WASM tests in a headless browser (Chrome or Firefox).
  ```bash
  wasm-pack test --chrome --headless
  wasm-pack test --firefox --headless
  ```

### Building for Production

For optimized production builds, use the `--release` flag.

```bash
wasm-pack build --target web --release
```

This command creates a optimized build in the `pkg/` directory, suitable for deployment.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [Bootstrap Documentation](https://getbootstrap.com/docs/)
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)

## 📊 Project Status

- [x] Basic WASM project setup
- [x] Modern Bootstrap UI layout
- [x] Calculator button layout
- [x] Basic project structure
- [x] Development environment setup
- [x] Calculator operations
- [x] Memory functions
- [x] Error handling
- [x] Modular code organization
- [ ] UI Interaction & State Management
- [ ] LLM Chatbot Integration
- [ ] Advanced Features & Refinement

## 📚 Documentation Standards

This project follows the official Rust documentation standards and conventions:

- [Rust Documentation Guide](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html) - Official guide for writing documentation
- [RFC 505: Documentation Conventions](https://rust-lang.github.io/rfcs/0505-api-comment-conventions.html) - Standard conventions for API documentation
- [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/) - Official Rust style guidelines

### Documentation Principles

1. Every public API has comprehensive documentation
2. All documentation includes examples
3. Comments follow the standard format:
   ```rust
   /// Short summary
   ///
   /// Detailed explanation
   ///
   /// # Examples
   ///
   /// ```rust
   /// // Example code
   /// ```
   ```
4. Module-level documentation uses `//!` comments
5. Function and type documentation uses `///` comments







