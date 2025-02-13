Packing repository using repomix...
Querying Gemini AI using gemini-2.0-flash-thinking-exp-01-21...
Okay, I will create a story-based tutorial series for building a Rust WebAssembly calculator, aimed at beginners, focusing on implementation with clear lessons, explanations, and code.

**Tutorial Series: Rusty's WebAssembly Calculator Adventure**

**Target Audience:** Beginners to Rust, WebAssembly, and web development.

**Story Premise:** Meet Rusty, a curious learner who wants to build a web calculator using Rust and WebAssembly. Follow Rusty's journey as he learns step-by-step, building his calculator from scratch.

**Tutorial Structure (Lessons):**

1.  **Lesson 1: Setting the Stage - Project Setup**
2.  **Lesson 2: Hello WASM - Your First Rust Function on the Web**
3.  **Lesson 3: Building the Calculator's Face - Basic UI with HTML**
4.  **Lesson 4: Making Buttons Clickable - JavaScript Interaction**
5.  **Lesson 5: Displaying Results from Rust - Connecting WASM to UI**
6.  **Lesson 6: Core Logic - Implementing Calculator Functions in Rust**
7.  **Lesson 7: Bringing it Together - Wiring Up the Calculator**

**(Start of Tutorial Series Content)**

---

# Rusty's WebAssembly Calculator Adventure - Tutorial Series

Welcome to Rusty's WebAssembly Calculator Adventure! In this series, we'll follow Rusty, a budding developer, as he builds a fully functional web calculator using the power of Rust and WebAssembly (WASM).  Don't worry if you're new to these technologies – we'll guide you through each step in a clear, easy-to-understand way. Let's start building!

## Lesson 1: Setting the Stage - Project Setup

**Rusty's Story:** "I've heard Rust is super fast and WebAssembly lets you run code in the browser. I want to combine them to make a cool calculator! But where do I even begin?"

**Lesson Objective:**  Set up your development environment and create the basic project structure for our Rust WASM calculator.

**What you'll need:**

*   **Rust Toolchain:** If you don't have Rust installed, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
*   **wasm-pack:**  This tool helps us build Rust code into WebAssembly. Install it by running `cargo install wasm-pack` in your terminal.
*   **Node.js and npm:**  Needed for development server and JavaScript tooling. Install from [https://nodejs.org/](https://nodejs.org/).

**Steps:**

1.  **Create a new Rust project:**
    Open your terminal and run:
    ```bash
    cargo new --lib rust-wasm-calc
    cd rust-wasm-calc
    ```
    This creates a new Rust library project named `rust-wasm-calc`.

2.  **Modify `Cargo.toml`:**
    Open `Cargo.toml` in your code editor. This file manages your Rust project's dependencies.  Replace its contents with the following:

    ```toml
    [package]
    name = "rust-wasm-calc"
    version = "0.1.0"
    edition = "2021"

    [lib]
    crate-type = ["cdylib", "rlib"]

    [dependencies]
    wasm-bindgen = "0.2"

    [dev-dependencies]
    wasm-bindgen-test = "0.3"
    ```
    *   `crate-type = ["cdylib", "rlib"]`:  Specifies that we want to build a dynamic library (`cdylib`) for WebAssembly and a regular Rust library (`rlib`).
    *   `wasm-bindgen = "0.2"`:  Includes the `wasm-bindgen` library, which is crucial for communication between Rust and JavaScript.

3.  **Create `index.html`:**
    In the root of your `rust-wasm-calc` project directory, create a new file named `index.html`.  This will be our basic webpage. Add the following HTML structure:

    ```html
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8">
        <title>Rust WASM Calculator</title>
    </head>
    <body>
        <div id="display">Loading WASM...</div>
        <script type="module">
            import init, { hello } from './pkg/rust_wasm_calc.js';

            async function run() {
                await init();
                const display = document.getElementById('display');
                const message = hello("WebAssembly");
                display.textContent = message;
            }

            run();
        </script>
    </body>
    </html>
    ```
    *   This is a very basic HTML page with a `div` for our calculator display and a `<script>` tag to load and run our WASM module.

4.  **Build the WASM module:**
    Run the following command in your terminal from the project root:
    ```bash
    wasm-pack build --target web
    ```
    This command compiles your Rust code to WebAssembly and generates JavaScript bindings in the `pkg` directory.

5.  **Run a local server:**
    To view your webpage, you need a simple HTTP server. If you don't have one, you can install `http-server` globally using npm:
    ```bash
    npm install -g http-server
    ```
    Then, start the server in your project root:
    ```bash
    http-server . -p 8080
    ```
    Open your browser and go to `http://localhost:8080`.

**Lesson Summary:**

Rusty has successfully set up his project! He created a Rust library, configured `Cargo.toml` for WASM, created a basic `index.html`, and built his first WASM module. Now he's ready to write some Rust code that interacts with the web.

**Next Up: Lesson 2 - Hello WASM!** We'll write our first Rust function that can be called from JavaScript and display a greeting on our webpage.

---

## Lesson 2: Hello WASM - Your First Rust Function on the Web

**Rusty's Story:** "Okay, project setup is done. Now, how do I actually write Rust code that the webpage can use? I want to see 'Hello' on the screen!"

**Lesson Objective:** Write a simple Rust function, compile it to WASM, and call it from JavaScript to display a greeting in the browser.

**Steps:**

1.  **Modify `src/lib.rs`:**
    Open `src/lib.rs`. This is where our Rust code lives. Replace its content with the following:

    ```rust
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn hello(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    ```
    *   `use wasm_bindgen::prelude::*;`: Imports necessary functionalities from the `wasm-bindgen` crate.
    *   `#[wasm_bindgen]`: This attribute is *key*. It tells `wasm-bindgen` to make this Rust function accessible from JavaScript.
    *   `pub fn hello(name: &str) -> String`:  Defines a public function named `hello` that takes a string slice (`&str`) as input and returns a `String`. This is our "Hello World" function!

2.  **Rebuild the WASM module:**
    In your terminal, run `wasm-pack build --target web` again. This recompiles your Rust code with the new `hello` function.

3.  **Check `index.html` (it should already be set up from Lesson 1):**
    Make sure your `index.html` `<script>` section looks like this:

    ```html
    <script type="module">
        import init, { hello } from './pkg/rust_wasm_calc.js';

        async function run() {
            await init();
            const display = document.getElementById('display');
            const message = hello("WebAssembly");
            display.textContent = message;
        }

        run();
    </script>
    ```
    *   `import { hello } from './pkg/rust_wasm_calc.js';`:  This line imports the `hello` function that we just created in Rust, making it available in our JavaScript code.
    *   `const message = hello("WebAssembly");`: Calls the `hello` function from Rust, passing "WebAssembly" as the `name` argument.
    *   `display.textContent = message;`: Sets the text content of the `div` with the ID `display` to the result of the `hello` function.

4.  **Refresh your browser:**
    Go back to your browser at `http://localhost:8080` and refresh the page.

**Success!** You should now see "Hello, WebAssembly!" displayed on your webpage instead of "Loading WASM...".

**Lesson Summary:**

Rusty wrote his first Rust function that's accessible from the web! He learned about the `#[wasm_bindgen]` attribute and how to call Rust functions from JavaScript.  He's one step closer to a calculator!

**Next Up: Lesson 3 - Building the Calculator's Face!** We'll create the user interface for our calculator using HTML and make it look like a real calculator.

---

## Lesson 3: Building the Calculator's Face - Basic UI with HTML

**Rusty's Story:** "Great, 'Hello' works! But a calculator needs buttons and a display that looks like a calculator, not just plain text. Time to work on the looks!"

**Lesson Objective:** Create the basic HTML structure for our calculator's user interface, including a display and buttons, using Bootstrap for styling.

**Steps:**

1.  **Update `index.html` with Bootstrap:**
    Replace the entire content of your `index.html` file with the following code. This incorporates Bootstrap for styling and sets up the calculator layout:

    ```html
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>Rust WASM Calculator</title>
        <!-- Bootstrap CSS -->
        <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
        <!-- Custom CSS (optional, for now) -->
        <style>
            .btn-calc { min-width: 60px; height: 60px; }
            #display { font-size: 2rem; text-align: right; padding: 1rem; }
            .calculator-container { max-width: 400px; margin: auto; } /* Center calculator */
        </style>
    </head>
    <body class="bg-light min-vh-100 d-flex align-items-center py-5">
        <div class="container">
            <div class="calculator-container">
                <div class="card shadow">
                    <div class="card-header bg-dark text-white">
                        <h5 class="mb-0">WASM Calculator</h5>
                    </div>
                    <div class="card-body p-0">
                        <div id="display" class="bg-light border-bottom">
                            0
                        </div>
                        <div class="p-3">
                            <div class="row g-2">
                                <!-- Row 1: Memory Operations -->
                                <div class="col-3"><button class="btn btn-secondary w-100 btn-calc">MC</button></div>
                                <div class="col-3"><button class="btn btn-secondary w-100 btn-calc">MR</button></div>
                                <div class="col-3"><button class="btn btn-secondary w-100 btn-calc">M+</button></div>
                                <div class="col-3"><button class="btn btn-secondary w-100 btn-calc">M-</button></div>
                                <!-- Row 2: Clear and Operations -->
                                <div class="col-6"><button class="btn btn-danger w-100 btn-calc">C</button></div>
                                <div class="col-3"><button class="btn btn-danger w-100 btn-calc">CE</button></div>
                                <div class="col-3"><button class="btn btn-primary w-100 btn-calc">/</button></div>
                                <!-- Row 3-6: Numbers and Operations -->
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">7</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">8</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">9</button></div>
                                <div class="col-3"><button class="btn btn-primary w-100 btn-calc">×</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">4</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">5</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">6</button></div>
                                <div class="col-3"><button class="btn btn-primary w-100 btn-calc">-</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">1</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">2</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">3</button></div>
                                <div class="col-3"><button class="btn btn-primary w-100 btn-calc">+</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">±</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">0</button></div>
                                <div class="col-3"><button class="btn btn-light w-100 btn-calc">.</button></div>
                                <div class="col-3"><button class="btn btn-primary w-100 btn-calc">=</button></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <!-- Bootstrap Bundle with Popper -->
        <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js"></script>
        <script type="module">
            import init, { hello } from './pkg/rust_wasm_calc.js';

            async function run() {
                await init();
                const display = document.getElementById('display');
                // For now, let's just keep displaying "Hello, WASM!"
                // We'll change this later to show calculator output
                const message = hello("WASM");
                display.textContent = message;
            }

            run();
        </script>
    </body>
    </html>
    ```

    *   **Bootstrap Integration:**  We've added Bootstrap CSS via CDN link in the `<head>`.  Bootstrap provides pre-built styles and layout components.
    *   **Calculator Structure:**  We use Bootstrap's grid system (`container`, `row`, `col`) and card component (`card`, `card-header`, `card-body`) to create the calculator layout.
    *   **Display (`#display`):**  A `div` with `id="display"` will act as our calculator's display.
    *   **Buttons:** We've added HTML `<button>` elements for all the calculator buttons (numbers, operations, memory, clear, etc.).  Bootstrap button classes (`btn`, `btn-primary`, `btn-light`, etc.) are used for styling.
    *   **Custom CSS:**  Basic CSS is added in `<style>` tag to adjust button size, display font size, and center the calculator.

2.  **Refresh your browser:**
    Refresh `http://localhost:8080` in your browser.

**Wow!**  Rusty now has a calculator interface that looks much more like a real calculator. It's styled with Bootstrap and has all the buttons we need.  However, the buttons don't do anything yet.

**Lesson Summary:**

Rusty has built the visual structure of his calculator using HTML and styled it with Bootstrap. He learned about Bootstrap's grid system and button components.  The calculator is starting to take shape!

**Next Up: Lesson 4 - Making Buttons Clickable!**  We'll use JavaScript to detect button clicks and start making the calculator interactive.

---

## Lesson 4: Making Buttons Clickable - JavaScript Interaction

**Rusty's Story:** "My calculator looks great, but it's just a picture right now!  I need to make these buttons actually *do* something when I click them. Time for some JavaScript magic!"

**Lesson Objective:** Use JavaScript to detect clicks on the calculator buttons and display the button's text content in the calculator display.

**Steps:**

1.  **Update `index.html` JavaScript:**
    Modify the `<script type="module">` section in your `index.html` to include event listeners for the buttons:

    ```html
    <script type="module">
        import init, { hello } from './pkg/rust_wasm_calc.js';

        async function run() {
            await init();
            const display = document.getElementById('display');
            display.textContent = '0'; // Initialize display to 0

            // Get all buttons within the calculator
            const buttons = document.querySelectorAll('.calculator-container button');

            buttons.forEach(button => {
                button.addEventListener('click', () => {
                    const buttonText = button.textContent;
                    if (display.textContent === '0') {
                        display.textContent = buttonText; // Replace '0' on first digit
                    } else {
                        display.textContent += buttonText; // Append to display
                    }
                });
            });
        }

        run();
    </script>
    ```

    *   `display.textContent = '0';`:  Initializes the calculator display to '0' when the page loads.
    *   `document.querySelectorAll('.calculator-container button');`: Selects all `<button>` elements that are descendants of the element with class `calculator-container`. This gets all our calculator buttons.
    *   `buttons.forEach(...)`:  Loops through each button.
    *   `button.addEventListener('click', ...)`:  Adds a click event listener to each button.
    *   **Inside the click handler:**
        *   `const buttonText = button.textContent;`: Gets the text content of the clicked button (e.g., "1", "+", "=").
        *   `if (display.textContent === '0') { ... } else { ... }`:  Handles the first digit input: if the display is '0', it replaces '0' with the button text. Otherwise, it appends the button text to the current display content.

2.  **Refresh your browser:**
    Refresh `http://localhost:8080` in your browser.

**Try it out!** Click on the number buttons and the operation buttons. You should see the button text appearing in the calculator display.  The '0' is replaced when you type the first digit, and subsequent button presses append to the display.

**Lesson Summary:**

Rusty made his calculator interactive! He used JavaScript to add event listeners to the buttons. Now, button clicks are detected, and the button's text is displayed.  The calculator is starting to respond to user input!

**Next Up: Lesson 5 - Displaying Results from Rust!** We'll start connecting our button clicks to Rust functions and display results calculated in Rust WASM.

---

## Lesson 5: Displaying Results from Rust - Connecting WASM to UI

**Rusty's Story:** "Clicking buttons is fun, but right now it's just showing what I clicked. I want to actually *calculate* things in Rust and show the result on the display! How do I get Rust to talk back to the webpage?"

**Lesson Objective:**  Modify our Rust code to perform a simple calculation (let's just add two numbers for now) and display the result in the calculator display after a button click.

**Steps:**

1.  **Update `src/lib.rs` with an `add` function:**
    Open `src/lib.rs` and add a new function called `add` that takes two numbers and returns their sum.  Also, let's keep our `hello` function for now.

    ```rust
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn hello(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    #[wasm_bindgen]
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    ```
    *   `#[wasm_bindgen] pub fn add(a: i32, b: i32) -> i32`:  Another `#[wasm_bindgen]` function, this time named `add`. It takes two 32-bit integers (`i32`) as input and returns their sum as an `i32`.

2.  **Rebuild the WASM module:**
    Run `wasm-pack build --target web` in your terminal again.

3.  **Update `index.html` JavaScript to call `add`:**
    Modify the `<script type="module">` section in `index.html`. We'll simplify the button click handling for now and just call our `add` function when *any* button is clicked (we'll refine this later).

    ```html
    <script type="module">
        import init, { hello, add } from './pkg/rust_wasm_calc.js'; // Import 'add'

        async function run() {
            await init();
            const display = document.getElementById('display');
            display.textContent = '0';

            const buttons = document.querySelectorAll('.calculator-container button');

            buttons.forEach(button => {
                button.addEventListener('click', () => {
                    // For now, let's just add 5 + 3 and display the result
                    const result = add(5, 3);
                    display.textContent = result.toString(); // Convert result to string
                });
            });
        }

        run();
    </script>
    ```
    *   `import { hello, add } from './pkg/rust_wasm_calc.js';`: Now we import both `hello` and `add` functions from our WASM module.
    *   `const result = add(5, 3);`: Inside the button click handler, we call the `add` function from Rust with the arguments `5` and `3`.
    *   `display.textContent = result.toString();`: We get the result from the `add` function (which is a number), convert it to a string using `.toString()`, and then display it in our calculator display.

4.  **Refresh your browser:**
    Refresh `http://localhost:8080`.

**Click any button on your calculator.**  No matter which button you click, the display should now show "8"!  This is because we are hardcoding `add(5, 3)` to be called on every button click.

**Lesson Summary:**

Rusty successfully called a Rust function (`add`) from his JavaScript code and displayed the result in the calculator! He's starting to see how Rust and JavaScript can work together to build calculator logic and UI.

**Next Up: Lesson 6 - Core Logic!** We'll implement more calculator operations (subtraction, multiplication, division) in Rust and prepare to use them in our calculator.

---

## Lesson 6: Core Logic - Implementing Calculator Functions in Rust

**Rusty's Story:** "Adding 5 and 3 is cool, but a real calculator needs to do more! Let's add subtraction, multiplication, and division to my Rust WASM module.  Then we'll have the core math part ready!"

**Lesson Objective:** Implement the basic arithmetic operations (addition, subtraction, multiplication, division) in Rust WASM.

**Steps:**

1.  **Update `src/lib.rs` with more functions:**
    Open `src/lib.rs` and add the functions for subtraction, multiplication, and division, along with the existing `hello` and `add` functions:

    ```rust
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn hello(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    #[wasm_bindgen]
    pub fn add(a: f64, b: f64) -> f64 { // Changed to f64 for decimals
        a + b
    }

    #[wasm_bindgen]
    pub fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    #[wasm_bindgen]
    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    #[wasm_bindgen]
    pub fn divide(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            // Handle division by zero (return NaN for now)
            std::f64::NAN
        } else {
            a / b
        }
    }
    ```
    *   **`f64` for Decimals:** We've changed the data type for `add`, `subtract`, `multiply`, and `divide` to `f64` (64-bit floating-point numbers). This allows our calculator to handle decimal numbers.
    *   **Division by Zero Handling:** In the `divide` function, we've added a check for division by zero. If `b` is 0, it returns `std::f64::NAN` (Not a Number) to avoid errors.

2.  **Rebuild the WASM module:**
    Run `wasm-pack build --target web` again.

3.  **No JavaScript changes needed yet:**
    For this lesson, we are just focusing on the Rust side. We'll connect these new functions to our JavaScript and UI in the next lesson.

**Lesson Summary:**

Rusty has implemented the core arithmetic logic for his calculator in Rust WASM! He added functions for addition, subtraction, multiplication, and division, and learned about using `f64` for decimal numbers and handling division by zero.  The calculator's brain is getting smarter!

**Next Up: Lesson 7 - Bringing it Together!**  We'll connect all the pieces: JavaScript button clicks will trigger the appropriate Rust functions, and the results will be displayed on the calculator. We'll build a fully functional basic calculator!

---

## Lesson 7: Bringing it Together - Wiring Up the Calculator

**Rusty's Story:** "I've got the buttons, I've got the math functions in Rust... Now, the final step!  I need to connect the button clicks to the right Rust functions and make this thing actually *calculate*!"

**Lesson Objective:** Connect the calculator UI buttons to the Rust WASM functions to perform calculations and display the results. This will create a basic working calculator.

**This lesson is a bit more involved, so we'll break it down further:**

**7.1.  JavaScript State Management:**

We need to keep track of the current number being entered, the operator selected, and the previous number.  Let's add some JavaScript variables to manage this state within our `<script type="module">` block in `index.html` (inside the `run` function):

```javascript
let currentNumber = '';
let operator = null;
let previousNumber = null;
```

*   `currentNumber`:  Stores the digits the user is currently typing.
*   `operator`: Stores the selected operator (+, -, *, /).
*   `previousNumber`: Stores the number entered before an operator is selected.

**7.2.  Update Button Click Handler:**

Modify the button click event listener in your JavaScript to handle different button types (numbers, operators, equals, clear).  Replace the existing button click handler with this more complex one:

```javascript
buttons.forEach(button => {
    button.addEventListener('click', () => {
        const buttonText = button.textContent;

        if (/[0-9]|\./.test(buttonText)) { // Number or decimal point
            if (display.textContent === '0' || display.textContent === 'Error') {
                display.textContent = buttonText;
            } else {
                display.textContent += buttonText;
            }
            currentNumber = display.textContent;
        } else if (/[\+\-×/]/.test(buttonText)) { // Operator
            if (operator !== null) {
                calculateResult(); // Perform pending calculation first
            }
            previousNumber = parseFloat(currentNumber);
            operator = buttonText;
            currentNumber = '';
            display.textContent = '0'; // Clear display for next number
        } else if (buttonText === '=') { // Equals
            calculateResult();
            operator = null;
            previousNumber = null;
        } else if (buttonText === 'C' || buttonText === 'CE') { // Clear
            clearCalculator();
        }
    });
});
```

*   **Number/Decimal Input (`/[0-9]|\./.test(buttonText)`):**
    *   If the button text is a digit or a decimal point, it appends it to the `display` and updates `currentNumber`.
*   **Operator Input (`/[\+\-×/]/.test(buttonText)`):**
    *   If an operator (+, -, ×, /) is clicked:
        *   If there's a pending operator (`operator !== null`), it first calls `calculateResult()` to complete the previous operation.
        *   Stores the `currentNumber` as `previousNumber`.
        *   Sets the `operator` to the clicked operator.
        *   Resets `currentNumber` and clears the `display` to prepare for the next number input.
*   **Equals Button (`buttonText === '='`):**
    *   When "=" is clicked, it calls `calculateResult()` to perform the calculation.
    *   Resets `operator` and `previousNumber`.
*   **Clear Buttons (`buttonText === 'C' || buttonText === 'CE'`):**
    *   Calls `clearCalculator()` to reset the calculator state.

**7.3.  Implement `calculateResult()` Function:**

Add the `calculateResult()` JavaScript function inside the `<script type="module">` block:

```javascript
function calculateResult() {
    if (operator === null || previousNumber === null) return;

    const num1 = previousNumber;
    const num2 = parseFloat(currentNumber);
    let result;

    switch (operator) {
        case '+': result = add(num1, num2); break;
        case '-': result = subtract(num1, num2); break;
        case '×': result = multiply(num1, num2); break;
        case '/': result = divide(num1, num2); break;
        default: return;
    }

    if (isNaN(result)) {
        display.textContent = 'Error';
    } else {
        display.textContent = result.toString();
    }
    currentNumber = display.textContent; // Update currentNumber with result
    operator = null; // Clear operator after calculation
    previousNumber = null; // Clear previousNumber
}
```

*   **`if (operator === null || previousNumber === null) return;`**:  Checks if we have both an operator and a previous number before attempting to calculate.
*   **`parseFloat(currentNumber)`**:  Parses the `currentNumber` string to a floating-point number.
*   **`switch (operator)`**:  Uses a `switch` statement to call the appropriate Rust WASM function (`add`, `subtract`, `multiply`, `divide`) based on the selected `operator`.
*   **`isNaN(result)`**: Checks if the result is `NaN` (Not a Number), which can happen in division by zero. If it is, it displays "Error".
*   **`display.textContent = result.toString();`**: Displays the calculated `result`.
*   Updates `currentNumber`, `operator`, and `previousNumber` after calculation.

**7.4.  Implement `clearCalculator()` Function:**

Add the `clearCalculator()` JavaScript function:

```javascript
function clearCalculator() {
    currentNumber = '';
    operator = null;
    previousNumber = null;
    display.textContent = '0';
}
```

*   Resets all state variables and clears the display to '0'.

**7.5. Rebuild and Refresh:**

*   Run `wasm-pack build --target web` one last time to ensure your Rust code is up-to-date.
*   Refresh your browser at `http://localhost:8080`.

**Test your calculator!** Try basic calculations like:

*   `5 + 3 =`  (should show 8)
*   `10 - 4 =` (should show 6)
*   `2 × 6 =`  (should show 12)
*   `15 / 3 =` (should show 5)
*   `5 / 0 =`  (should show "Error")

**Congratulations!** Rusty has built a basic, functional web calculator using Rust and WebAssembly! He connected the UI buttons to Rust functions and made calculations happen in the browser using WASM.

**Lesson Summary:**

Rusty brought all the pieces together! He implemented JavaScript state management, updated the button click handler to perform calculations using Rust WASM functions, and created a working calculator. He learned how JavaScript can interact with Rust WASM to build web applications.

**Where to go next?**

This is just the beginning! Rusty (and you!) can now expand this calculator with more features:

*   **Memory functions (MC, MR, M+, M-)**
*   **More advanced operations (square root, percentages, etc.)**
*   **Error handling improvements**
*   **Better UI/UX**
*   **Testing (Unit tests in Rust, integration tests in JavaScript)**

This tutorial series gave you a foundation for building web applications with Rust and WebAssembly. Keep experimenting and building!  Happy coding!

**(End of Tutorial Series Content)**

---

This completes the story-based tutorial series. It covers the basic implementation of a Rust WASM calculator for beginners, focusing on step-by-step instructions and explanations. It avoids testing to keep the focus on the core implementation as requested.