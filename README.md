# Rust WASM Calculator

A WebAssembly calculator built with Rust and Bootstrap 5, following test-driven development practices.

## Project Structure

```
rust-wasm-calc/
├── Cargo.toml         # Rust package configuration
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

### Phase 0: Basic Functionality
- [x] Project setup and configuration
- [x] WASM binding implementation
- [x] Bootstrap UI implementation
- [x] Testing infrastructure

### Phase 1: Basic Operations
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
- Square design using CSS
- Bootstrap Icons for symbols
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

## Testing

The project uses a test-driven development approach with multiple test types and environments.

### Test Types

1. **Unit Tests**
   ```rust
   // Example test structure (to be implemented)
   #[test]
   fn test_add() {
       let result = add(2.0, 3.0);
       assert_eq!(result, 5.0);
   }
   ```

2. **WASM Browser Tests**
   ```rust
   // Example test structure (to be implemented)
   #[wasm_bindgen_test]
   fn test_add_in_browser() {
       let result = add(2.0, 3.0);
       assert_eq!(result, 5.0);
   }
   ```

3. **Integration Tests** (in `tests/web.rs`)
   ```rust
   // Example test structure (to be implemented)
   #[wasm_bindgen_test]
   fn test_calculator_display() {
       // Test calculator display updates
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

The calculator will implement the following features (currently in development):

### Planned Basic Operations
```rust
// To be implemented
pub fn add(a: f64, b: f64) -> f64;
pub fn subtract(a: f64, b: f64) -> f64;
pub fn multiply(a: f64, b: f64) -> f64;
pub fn divide(a: f64, b: f64) -> Result<f64, String>;
```

### Planned Memory Operations
```rust
// To be implemented
pub fn memory_add(value: f64);
pub fn memory_subtract(value: f64);
pub fn memory_recall() -> f64;
pub fn memory_clear();
```

### Current Features
- [x] Basic WASM project setup
- [x] Modern Bootstrap UI layout
- [x] Calculator button layout
- [x] Basic project structure
- [x] Development environment setup
- [ ] Calculator operations
- [ ] Memory functions
- [ ] Error handling

## Setup

1. **Clone and Enter the Repository:**
```bash
git clone https://github.com/your-username/rust-wasm-calc.git
cd rust-wasm-calc
```

2. **Install Prerequisites:**
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install http-server (for development)
npm install -g http-server
```

3. **Build the Project:**
```bash
# Build WASM module
wasm-pack build --target web

# The output will be in the pkg/ directory
```

4. **Run Development Server:**
```bash
# Start the server
http-server . -p 8080

# Open http://localhost:8080 in your browser
```

5. **Run Tests:**
```bash
# Run Rust unit tests
cargo test

# Run WASM tests in headless Chrome
wasm-pack test --headless --chrome
```

## Development

### Project Configuration

The project uses the following key dependencies:
- `wasm-bindgen`: For Rust and JavaScript interop
- `console_error_panic_hook`: For better error handling in browser
- `wasm-bindgen-test`: For testing WASM code
- Bootstrap 5.3.0: For UI components
- Bootstrap Icons: For calculator symbols

### Development Workflow

1. Write tests first (following TDD)
2. Implement the feature in Rust
3. Build WASM module
4. Test in browser
5. Commit changes

### Building for Production

For production builds:

```bash
# Build with optimizations
wasm-pack build --target web --release

# The optimized output will be in the pkg/ directory
```
