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

#### Rust Backend

- **calculator**: Core calculator functionality
  - **operations.rs**: Basic arithmetic operations
  - **memory.rs**: Memory-related functions
  - **mod.rs**: Module exports and organization
- **state**: State management for the calculator
  - **types.rs**: Type definitions (Operation enum, CalculatorState struct)
  - **operations.rs**: Methods for manipulating calculator state
  - **mod.rs**: Module exports and organization
- **errors.rs**: Error handling and standardized error types
- **lib.rs**: Main library entry point and WebAssembly exports

#### JavaScript Frontend

- **js/calculator.js**: Main entry point for the calculator UI
- **js/calculator/**: Modular JavaScript components
  - **controller.js**: CalculatorController class for UI-WASM interaction
  - **events.js**: Event handling for buttons and keyboard input
  - **display.js**: Display formatting and updates
  - **memory.js**: Memory operations interface
- **js/chatbot.js**: Chatbot interface implementation
- **js/navigation.js**: Navigation system for switching between views

#### HTML/CSS Structure

- **index.html**: Main HTML file with responsive layout
- **custom.css**: Custom styles for calculator and chatbot interfaces

#### Tests

- **tests/calculator/**: Tests for calculator operations
  - **operations_tests.rs**: Tests for arithmetic operations
  - **memory_tests.rs**: Tests for memory functions
- **tests/state/**: Tests for state management
  - **types_tests.rs**: Tests for state type definitions
  - **operations_tests.rs**: Tests for state operations
  - **mod.rs**: Integration tests for state management
- **tests/web.rs**: WASM-specific integration tests

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

#### State Management

The calculator uses a state management system to track the current state of the calculator:

- **`CalculatorState`**: Main state container for the calculator
  - **`display_value()`**: Gets the current display value
  - **`input_digit(digit: u8)`**: Inputs a digit (0-9)
  - **`input_decimal()`**: Inputs a decimal point
  - **`toggle_sign()`**: Toggles the sign of the current value
  - **`backspace()`**: Removes the last character
  - **`set_operation(operation: Operation)`**: Sets the current operation
  - **`calculate()`**: Performs the calculation
  - **`clear()`**: Clears the calculator state
  - **`clear_entry()`**: Clears the current entry

- **`Operation`**: Enum representing different operations
  - `None`: No operation
  - `Add`: Addition
  - `Subtract`: Subtraction
  - `Multiply`: Multiplication
  - `Divide`: Division

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

### JavaScript Architecture

The JavaScript frontend follows a modular architecture:

1. **Main Module (`calculator.js`)**: 
   - Initializes the calculator
   - Connects WASM functions to the UI
   - Serves as the entry point

2. **Controller (`controller.js`)**: 
   - Manages the interaction between UI and WASM
   - Handles user input and updates the display
   - Maintains the calculator state

3. **Events (`events.js`)**: 
   - Handles button click events
   - Manages keyboard input
   - Dynamically adds UI elements

4. **Display (`display.js`)**: 
   - Formats values for display
   - Updates the display element
   - Handles error display

5. **Memory (`memory.js`)**: 
   - Provides a clean interface to WASM memory functions
   - Manages memory operations

6. **Chatbot (`chatbot.js`)**:
   - Implements chat interface for natural language calculations
   - Processes user messages and generates responses
   - Integrates with calculator functions
   - Manages conversation history

7. **Navigation (`navigation.js`)**:
   - Handles switching between different application views
   - Manages URL hash-based routing
   - Updates active navigation state

**Example Usage (JavaScript):**

```javascript
import init, { hello, add, subtract, multiply, divide, memory_store, memory_recall } from './pkg/rust_wasm_calc.js';
import { initCalculator } from './js/calculator.js';
import { initChatbot } from './js/chatbot.js';
import { initNavigation } from './js/navigation.js';

async function run() {
  await init();
  
  // Create calculator functions object
  const calculatorFunctions = {
    memory_store,
    memory_recall,
    memory_clear,
    memory_add,
    memory_subtract
  };
  
  // Initialize the calculator UI
  const calculator = initCalculator(calculatorFunctions);
  
  // Initialize the chatbot UI
  initChatbot(calculatorFunctions);
  
  // Initialize navigation
  initNavigation();
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
- [x] UI Interaction & State Management
- [x] Navigation system with landing page
- [x] Basic chatbot interface
- [ ] LLM API integration
- [ ] Advanced Features & Refinement

## 🌟 Features

- **Traditional Calculator Interface**: Standard calculator with memory functions
- **Chatbot Calculator Interface**: Natural language interface for calculations
- **Responsive Design**: Works on desktop and mobile devices
- **Memory Functions**: Store, recall, add, and subtract values from memory
- **Error Handling**: Comprehensive error handling for all operations

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







