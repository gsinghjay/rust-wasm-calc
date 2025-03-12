/**
 * Calculator Display Module
 * 
 * This module handles the display formatting and updates for the calculator UI.
 */

/**
 * Gets the display element from the DOM
 * 
 * @returns {HTMLElement} The display element
 */
export function getDisplayElement() {
  return document.getElementById('display');
}

/**
 * Formats a number for display
 * 
 * @param {number|string} value - The value to format
 * @returns {string} The formatted value
 */
export function formatDisplayValue(value) {
  // If it's already a string, return it (likely an error message)
  if (typeof value === 'string' && isNaN(parseFloat(value))) {
    return value;
  }
  
  // Convert to number if it's a string
  const numValue = typeof value === 'string' ? parseFloat(value) : value;
  
  // Check if it's a whole number
  if (Number.isInteger(numValue)) {
    return numValue.toString();
  }
  
  // Format floating point numbers
  return numValue.toString();
}

/**
 * Updates the display with a new value
 * 
 * @param {HTMLElement} displayElement - The display element
 * @param {string|number} value - The value to display
 */
export function updateDisplay(displayElement, value) {
  displayElement.textContent = formatDisplayValue(value);
}

/**
 * Sets the display to show an error message
 * 
 * @param {HTMLElement} displayElement - The display element
 * @param {string} message - The error message
 */
export function showError(displayElement, message) {
  displayElement.textContent = `Error: ${message}`;
  displayElement.classList.add('error');
}

/**
 * Clears any error state from the display
 * 
 * @param {HTMLElement} displayElement - The display element
 */
export function clearError(displayElement) {
  displayElement.classList.remove('error');
} 