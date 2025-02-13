# Rust WASM Calculator

A WebAssembly calculator built with Rust and Bootstrap 5, following test-driven development practices.

## Project Structure

```
rust-wasm-calc/
├── Cargo.toml          # Rust package configuration
├── src/
│   └── lib.rs         # Rust library code with WASM bindings
├── tests/
│   └── web.rs         # Integration tests for browser environment
├── pkg/               # Generated WASM package (after build)
├── custom.css         # Custom styles for the calculator
└── index.html         # Web interface with Bootstrap 5
```

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://rustup.rs/) (latest stable version)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (for running the development server)
- A modern browser (Chrome, Firefox, or Safari) for running tests

## Development Roadmap

### Phase 1: Basic Operations
- [x] Project setup and configuration
- [x] WASM binding implementation
- [x] Bootstrap UI implementation
- [x] Testing infrastructure
- [x] Dark/Light theme support
- [ ] Addition operation
- [ ] Subtraction operation
- [ ] Multiplication operation
- [ ] Division operation

### Phase 2: Advanced Features
- [ ] Memory functions (M+, M-, MR, MC)
- [ ] History of calculations
- [ ] Scientific calculator functions
- [ ] Keyboard input support
- [ ] Mobile-first responsive design

### Phase 3: Optimizations
- [ ] Performance benchmarking
- [ ] WASM size optimization
- [ ] Error handling improvements
- [ ] Accessibility features (WCAG 2.1)

## UI Implementation

The calculator uses Bootstrap 5.3.0 for its user interface with the following features:

### Components
- Square design using `rounded-0` class
- Bootstrap Icons for symbols
- Bootstrap's theme system for dark/light modes
- Bootstrap's grid system for button layout
- Bootstrap's utility classes for spacing and alignment

### Custom Styling
Custom styles are maintained in `custom.css` and extend Bootstrap's functionality:
```css
/* Calculator container */
.calculator-container {
    max-width: 400px;
    margin: auto;
}

/* Square design for all calculator elements */
.calculator-container .card,
.calculator-container .card-header,
.calculator-container .btn,
.calculator-container #display {
    border-radius: 0 !important;
}

/* Calculator buttons */
.btn-calc {
    min-width: 60px;
    height: 60px;
}

/* Calculator display */
#display {
    font-family: 'Roboto Mono', monospace;
    height: 80px;
    font-size: 2rem;
    text-align: right;
    padding: 1rem;
}
```

### Theme Support
- Light/Dark mode toggle
- System-wide theme consistency
- Bootstrap's `data-bs-theme` attribute
- Automatic theme switching

## Testing

The project uses a comprehensive testing approach with multiple test types and environments.

### Test Types

1. **Unit Tests**
   ```rust
   #[test]
   fn test_hello_rust() {
       let result = hello("Rust");
       assert_eq!(result, "Hello, Rust!");
   }
   ```

2. **WASM Browser Tests**
   ```rust
   #[wasm_bindgen_test]
   fn test_hello_wasm_browser() {
       let result = hello("WASM");
       assert_eq!(result, "Hello, WASM!");
   }
   ```

3. **Integration Tests** (in `tests/web.rs`)
   ```rust
   #[wasm_bindgen_test]
   fn test_hello_unicode() {
       let result = hello("🌍");
       assert_eq!(result, "Hello, 🌍!");
   }
   ```

### Running Tests

```bash
# Run regular Rust tests
cargo test

# Run tests in Node.js
wasm-pack test --node

# Run tests in Chrome
wasm-pack test --chrome --headless

# Run tests in Firefox
wasm-pack test --firefox --headless
```

## Calculator Functionality

The calculator implements the following features:

### Basic Operations
```rust
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

The project is configured with:
- `wasm-bindgen`: For Rust-JavaScript interop
- `console_error_panic_hook`: For better error handling
- `wasm-bindgen-test`: For WASM-specific testing
- Bootstrap 5.3.0: For UI components
- Bootstrap Icons: For calculator symbols

### Current Features

- Basic WASM setup with Rust
- Modern Bootstrap UI
- Dark/Light theme toggle
- Memory operation buttons
- Comprehensive test infrastructure
- Error handling
- Unicode support

## Building for Production

To build the project for production:

```bash
wasm-pack build --target web --release
```

This will generate optimized WASM code in the `pkg` directory.

## Contributing

1. Write tests first (TDD approach)
2. Follow Bootstrap conventions
3. Maintain square design (use `rounded-0`)
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Documentation](https://rustwasm.github.io/docs/wasm-bindgen/)
- [Bootstrap Documentation](https://getbootstrap.com/docs/5.3/)
- [Bootstrap Icons](https://icons.getbootstrap.com/) 