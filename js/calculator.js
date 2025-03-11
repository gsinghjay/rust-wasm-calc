/**
 * Calculator UI interaction module
 * 
 * This module handles the interaction between the UI and the WASM calculator.
 * It connects button clicks to WASM functions and updates the display.
 */

import { CalculatorState, Operation } from '../pkg/rust_wasm_calc.js';

/**
 * Calculator controller class
 * 
 * Manages the interaction between the UI and the WASM calculator state.
 */
export class CalculatorController {
  /**
   * Creates a new calculator controller
   * 
   * @param {HTMLElement} displayElement - The element to display calculator output
   */
  constructor(displayElement) {
    this.displayElement = displayElement;
    this.state = new CalculatorState();
    this.updateDisplay();
  }

  /**
   * Updates the display with the current calculator state
   */
  updateDisplay() {
    this.displayElement.textContent = this.state.display_value;
  }

  /**
   * Handles digit button clicks
   * 
   * @param {number} digit - The digit that was clicked (0-9)
   */
  handleDigit(digit) {
    this.state.input_digit(digit);
    this.updateDisplay();
  }

  /**
   * Handles decimal point button click
   */
  handleDecimal() {
    this.state.input_decimal();
    this.updateDisplay();
  }

  /**
   * Handles operation button clicks
   * 
   * @param {string} op - The operation symbol (+, -, ×, /)
   */
  handleOperation(op) {
    let operation;
    switch (op) {
      case '+':
        operation = Operation.Add;
        break;
      case '-':
        operation = Operation.Subtract;
        break;
      case '×':
        operation = Operation.Multiply;
        break;
      case '/':
        operation = Operation.Divide;
        break;
      default:
        return;
    }
    
    this.state.set_operation(operation);
    this.updateDisplay();
  }

  /**
   * Handles equals button click
   */
  handleEquals() {
    this.state.calculate();
    this.updateDisplay();
  }

  /**
   * Handles clear button click
   */
  handleClear() {
    this.state.clear();
    this.updateDisplay();
  }

  /**
   * Handles clear entry button click
   */
  handleClearEntry() {
    this.state.clear_entry();
    this.updateDisplay();
  }

  /**
   * Handles toggle sign button click
   */
  handleToggleSign() {
    this.state.toggle_sign();
    this.updateDisplay();
  }

  /**
   * Handles backspace button click
   */
  handleBackspace() {
    this.state.backspace();
    this.updateDisplay();
  }

  /**
   * Handles memory store button click
   */
  handleMemoryStore() {
    const value = parseFloat(this.state.display_value);
    if (!isNaN(value)) {
      window.memory_store(value);
    }
  }

  /**
   * Handles memory recall button click
   */
  handleMemoryRecall() {
    this.state.clear();
    const value = window.memory_recall();
    // Format the value to avoid unnecessary decimal places
    if (value === Math.floor(value)) {
      this.displayElement.textContent = Math.floor(value).toString();
    } else {
      this.displayElement.textContent = value.toString();
    }
  }

  /**
   * Handles memory add button click
   */
  handleMemoryAdd() {
    const value = parseFloat(this.state.display_value);
    if (!isNaN(value)) {
      window.memory_add(value);
    }
  }

  /**
   * Handles memory subtract button click
   */
  handleMemorySubtract() {
    const value = parseFloat(this.state.display_value);
    if (!isNaN(value)) {
      window.memory_subtract(value);
    }
  }

  /**
   * Handles memory clear button click
   */
  handleMemoryClear() {
    window.memory_clear();
  }

  /**
   * Handles keyboard input
   * 
   * @param {KeyboardEvent} event - The keyboard event
   * @returns {boolean} - Whether the key was handled
   */
  handleKeyDown(event) {
    // Prevent default behavior for calculator keys
    let handled = true;

    switch (event.key) {
      case '0':
      case '1':
      case '2':
      case '3':
      case '4':
      case '5':
      case '6':
      case '7':
      case '8':
      case '9':
        this.handleDigit(parseInt(event.key, 10));
        break;
      case '.':
        this.handleDecimal();
        break;
      case '+':
        this.handleOperation('+');
        break;
      case '-':
        this.handleOperation('-');
        break;
      case '*':
      case 'x':
      case '×':
        this.handleOperation('×');
        break;
      case '/':
        this.handleOperation('/');
        break;
      case '=':
      case 'Enter':
        this.handleEquals();
        break;
      case 'Escape':
        this.handleClear();
        break;
      case 'Delete':
        this.handleClearEntry();
        break;
      case 'Backspace':
        this.handleBackspace();
        break;
      default:
        handled = false;
    }

    if (handled) {
      event.preventDefault();
    }

    return handled;
  }
}

/**
 * Initializes the calculator UI
 * 
 * @param {Object} wasmModule - The WASM module containing calculator functions
 */
export function initCalculator(wasmModule) {
  const display = document.getElementById('display');
  const calculator = new CalculatorController(display);

  // Expose memory functions globally for the controller to use
  window.memory_store = wasmModule.memory_store;
  window.memory_recall = wasmModule.memory_recall;
  window.memory_clear = wasmModule.memory_clear;
  window.memory_add = wasmModule.memory_add;
  window.memory_subtract = wasmModule.memory_subtract;

  // Set up button click handlers
  document.querySelectorAll('.btn-calc').forEach(button => {
    button.addEventListener('click', () => {
      const text = button.textContent;
      
      // Handle different button types
      if (text.match(/[0-9]/)) {
        calculator.handleDigit(parseInt(text, 10));
      } else if (text === '.') {
        calculator.handleDecimal();
      } else if (text === '+' || text === '-' || text === '×' || text === '/') {
        calculator.handleOperation(text);
      } else if (text === '=') {
        calculator.handleEquals();
      } else if (text === 'C') {
        calculator.handleClear();
      } else if (text === 'CE') {
        calculator.handleClearEntry();
      } else if (text === '±') {
        calculator.handleToggleSign();
      } else if (text === 'MC') {
        calculator.handleMemoryClear();
      } else if (text === 'MR') {
        calculator.handleMemoryRecall();
      } else if (text === 'M+') {
        calculator.handleMemoryAdd();
      } else if (text === 'M-') {
        calculator.handleMemorySubtract();
      }
    });
  });

  // Set up keyboard event handling
  document.addEventListener('keydown', (event) => {
    calculator.handleKeyDown(event);
  });

  // Add a backspace button
  const backspaceButton = document.createElement('button');
  backspaceButton.className = 'btn btn-secondary w-100 btn-calc';
  backspaceButton.innerHTML = '<i class="bi bi-backspace"></i>';
  backspaceButton.addEventListener('click', () => {
    calculator.handleBackspace();
  });

  // Find a good place to add the backspace button
  const clearEntryButton = Array.from(document.querySelectorAll('.btn-calc')).find(btn => btn.textContent === 'CE');
  if (clearEntryButton && clearEntryButton.parentNode) {
    const backspaceCol = document.createElement('div');
    backspaceCol.className = 'col-3';
    backspaceCol.appendChild(backspaceButton);
    
    // Insert after CE button
    clearEntryButton.parentNode.parentNode.insertBefore(backspaceCol, clearEntryButton.parentNode.nextSibling);
  }

  return calculator;
} 