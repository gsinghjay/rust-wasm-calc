/**
 * Calculator Events Module
 * 
 * This module handles event binding and delegation for the calculator UI.
 * It connects DOM events to the calculator controller methods.
 */

/**
 * Binds click events to calculator buttons
 * 
 * @param {Object} calculator - The calculator controller instance
 */
export function bindButtonEvents(calculator) {
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
}

/**
 * Binds keyboard events to the calculator
 * 
 * @param {Object} calculator - The calculator controller instance
 */
export function bindKeyboardEvents(calculator) {
  // Disabled to prevent interference with chatbot input
  // document.addEventListener('keydown', (event) => {
  //   calculator.handleKeyDown(event);
  // });
  
  // This function is now a no-op
  console.log('Keyboard events for calculator are disabled');
}

/**
 * Creates and adds a backspace button to the calculator UI
 * 
 * @param {Object} calculator - The calculator controller instance
 */
export function addBackspaceButton(calculator) {
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
} 