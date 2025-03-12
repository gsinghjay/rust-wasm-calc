/**
 * Calculator Controller Module
 * 
 * This module contains the CalculatorController class which manages the
 * interaction between the UI and the WASM calculator state.
 */

import { CalculatorState, Operation } from '../../pkg/rust_wasm_calc.js';

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
    this.displayElement.textContent = this.state.display_value();
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
    
    // Map the button symbol to the Operation enum
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
        console.error(`Unknown operation: ${op}`);
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
    // Get the current display value as a number
    const value = parseFloat(this.state.display_value());
    
    // Store it in memory
    window.memory_store(value);
  }

  /**
   * Handles memory recall button click
   */
  handleMemoryRecall() {
    // Clear the current state
    this.state.clear_entry();
    
    // Get the value from memory
    const value = window.memory_recall();
    
    // Set the display to show the memory value
    this.displayElement.textContent = value.toString();
  }

  /**
   * Handles memory add button click
   */
  handleMemoryAdd() {
    // Get the current display value as a number
    const value = parseFloat(this.state.display_value());
    
    // Add it to memory
    window.memory_add(value);
  }

  /**
   * Handles memory subtract button click
   */
  handleMemorySubtract() {
    // Get the current display value as a number
    const value = parseFloat(this.state.display_value());
    
    // Subtract it from memory
    window.memory_subtract(value);
  }

  /**
   * Handles memory clear button click
   */
  handleMemoryClear() {
    // Clear the memory
    window.memory_clear();
  }

  /**
   * Handles keyboard events
   * 
   * @param {KeyboardEvent} event - The keyboard event
   */
  handleKeyDown(event) {
    // Prevent default behavior for calculator keys
    if (
      (event.key >= '0' && event.key <= '9') ||
      event.key === '.' ||
      event.key === '+' ||
      event.key === '-' ||
      event.key === '*' ||
      event.key === '/' ||
      event.key === 'Enter' ||
      event.key === 'Escape' ||
      event.key === 'Backspace'
    ) {
      event.preventDefault();
    }
    
    // Handle different key presses
    if (event.key >= '0' && event.key <= '9') {
      this.handleDigit(parseInt(event.key, 10));
    } else if (event.key === '.') {
      this.handleDecimal();
    } else if (event.key === '+') {
      this.handleOperation('+');
    } else if (event.key === '-') {
      this.handleOperation('-');
    } else if (event.key === '*') {
      this.handleOperation('×');
    } else if (event.key === '/') {
      this.handleOperation('/');
    } else if (event.key === 'Enter' || event.key === '=') {
      this.handleEquals();
    } else if (event.key === 'Escape') {
      this.handleClear();
    } else if (event.key === 'Backspace') {
      this.handleBackspace();
    }
  }
} 