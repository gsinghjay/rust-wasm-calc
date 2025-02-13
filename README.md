# Rust WASM Calculator

A WebAssembly calculator built with Rust, following test-driven development practices.

## Project Structure

```
rust-wasm-calc/
├── Cargo.toml          # Rust package configuration
├── src/
│   └── lib.rs         # Rust library code with WASM bindings
├── pkg/               # Generated WASM package (after build)
└── index.html         # Web interface
```

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://rustup.rs/) (latest stable version)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (for running the development server)

## Development Roadmap

### Phase 1: Basic Operations
- [x] Project setup and configuration
- [x] WASM binding implementation
- [x] Basic web interface
- [ ] Addition operation
- [ ] Subtraction operation
- [ ] Multiplication operation
- [ ] Division operation

### Phase 2: Advanced Features
- [ ] Memory functions (M+, M-, MR, MC)
- [ ] History of calculations
- [ ] Scientific calculator functions
- [ ] Keyboard input support
- [ ] Responsive mobile design

### Phase 3: Optimizations
- [ ] Performance benchmarking
- [ ] WASM size optimization
- [ ] Error handling improvements
- [ ] Accessibility features

## Calculator Functionality

The calculator will implement the following features:

### Basic Operations
```rust
// Example of planned functionality
pub fn add(a: f64, b: f64) -> f64;
pub fn subtract(a: f64, b: f64) -> f64;
pub fn multiply(a: f64, b: f64) -> f64;
pub fn divide(a: f64, b: f64) -> Result<f64, String>;
```

### Memory Operations
```rust
pub fn memory_add(value: f64);
pub fn memory_subtract(value: f64);
pub fn memory_recall() -> f64;
pub fn memory_clear();
```

## Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-wasm-calc
```

2. Build the WASM module:
```bash
wasm-pack build --target web
```

3. Start the development server:
```bash
npm install -g http-server
http-server . -p 8080
```

4. Open your browser and navigate to `http://localhost:8080`

## Development

### Project Configuration

The project is configured in `Cargo.toml` with the following key dependencies:
- `wasm-bindgen`: For Rust-JavaScript interop
- `console_error_panic_hook`: For better error handling in the browser
- Test dependencies for WASM testing

### Testing

The project uses test-driven development with both unit tests and browser-based tests. Tests are written using `wasm-bindgen-test`.

To run tests:
```bash
# Run tests in Chrome
wasm-pack test --chrome

# Run tests in Node.js
wasm-pack test --node
```

### Current Features

- Basic WASM setup with Rust
- Hello World functionality
- Browser integration
- Test infrastructure
- Error handling

### Web Interface

The web interface (`index.html`) provides:
- Modern, responsive design
- Error handling for WASM operations
- Clean user interface for future calculator functionality

## Building for Production

To build the project for production:

```bash
wasm-pack build --target web --release
```

This will generate optimized WASM code in the `pkg` directory.

## Contributing

1. Write tests first (TDD approach)
2. Implement features
3. Ensure all tests pass
4. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Documentation](https://rustwasm.github.io/docs/wasm-bindgen/) 