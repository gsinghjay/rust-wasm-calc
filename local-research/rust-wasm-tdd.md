Querying Perplexity AI using sonar-pro for: rust wasm hello world best practices with test driven development
To create a Rust WebAssembly "Hello World" project with test-driven development, follow these best practices:

1. Set up the project:

```bash
cargo new --lib hello-wasm
cd hello-wasm
```

2. Configure Cargo.toml:

```toml
[package]
name = "hello-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

3. Write the main function in src/lib.rs:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

4. Add tests in src/lib.rs:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }
}
```

5. Run tests:

```bash
wasm-pack test --node
```

6. Build the project:

```bash
wasm-pack build --target web
```

7. Create an HTML file to use the WebAssembly module:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Hello WASM</title>
</head>
<body>
    <script type="module">
        import init, { greet } from './pkg/hello_wasm.js';
        
        async function run() {
            await init();
            const result = greet("WASM");
            console.log(result);
        }
        
        run();
    </script>
</body>
</html>
```

8. Serve the project using a local web server and open it in a browser to see the result[1][3][4].

This approach follows test-driven development by writing tests before implementing functionality, uses wasm-bindgen for JavaScript interop, and follows Rust WebAssembly best practices[1][2][4].