# Rust WASM Calculator

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![wasm-pack](https://img.shields.io/badge/wasm--pack-latest-blue.svg)](https://rustwasm.github.io/wasm-pack/)
[![Bootstrap](https://img.shields.io/badge/bootstrap-5.3.0-purple.svg)](https://getbootstrap.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Rust WASM Calculator is a WebAssembly calculator application. It is built using Rust and Bootstrap 5. The project follows test-driven development practices.

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

### API

#### `hello(name: &str) -> String`

- **Description:**  Returns a greeting string.
- **Parameters:**
    - `name`:  The name to include in the greeting.
- **Returns:**  A string formatted as "Hello, {name}!".

**Example Usage (JavaScript):**

```javascript
import init, { hello } from './pkg/rust_wasm_calc.js';

async function run() {
  await init();
  const message = hello("User");
  console.log(message); // Output: Hello, User!
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
- [ ] Calculator operations
- [ ] Memory functions
- [ ] Error handling

## 🔗 Links

- [Project Roadmap](local-research/initial-roadmap.md)
- [Documentation](docs/)
- [Issue Tracker](../../issues)
- [Pull Requests](../../pulls)







