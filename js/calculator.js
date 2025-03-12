/**
 * Calculator UI main module
 * 
 * This module initializes the calculator UI and connects it to the WASM backend.
 * It serves as the entry point for the calculator application.
 */

import { CalculatorController } from './calculator/controller.js';
import { bindButtonEvents, bindKeyboardEvents, addBackspaceButton } from './calculator/events.js';
import { getDisplayElement } from './calculator/display.js';
import { initMemory } from './calculator/memory.js';

/**
 * Initializes the calculator with WASM functions
 * 
 * @param {Object} wasmFunctions - Object containing WASM calculator functions
 * @returns {Object} The calculator controller instance
 */
export function initCalculator(wasmFunctions) {
  // Initialize memory module
  const memory = initMemory(wasmFunctions);
  
  // Make memory functions available globally for the controller
  window.memory_store = memory.store;
  window.memory_recall = memory.recall;
  window.memory_clear = memory.clear;
  window.memory_add = memory.add;
  window.memory_subtract = memory.subtract;
  
  // Get the display element
  const display = getDisplayElement();
  
  // Create the calculator controller
  const calculator = new CalculatorController(display);
  
  // Bind events
  bindButtonEvents(calculator);
  // Completely disabling keyboard events to prevent interference with chatbot input
  // bindKeyboardEvents(calculator);
  addBackspaceButton(calculator);
  
  return calculator;
} 