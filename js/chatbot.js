/**
 * Chatbot module for the Rust WASM Calculator
 * Handles chat interface and integration with calculator functions
 */

/**
 * Initialize the chatbot functionality
 * @param {Object} calculatorFunctions - Object containing WASM calculator functions
 */
export function initChatbot(calculatorFunctions) {
    // DOM elements
    const chatForm = document.getElementById('chat-form');
    const chatInput = document.getElementById('chat-input');
    const chatMessages = document.getElementById('chat-messages');
    const chatStatus = document.getElementById('chat-status');
    
    // Chat history for context
    let chatHistory = [];
    
    /**
     * Add a message to the chat interface
     * @param {string} content - The message content
     * @param {string} role - The role of the message sender ('user' or 'assistant')
     */
    function addMessage(content, role) {
        // Create message elements
        const messageDiv = document.createElement('div');
        messageDiv.classList.add('message', role);
        
        const contentDiv = document.createElement('div');
        contentDiv.classList.add('message-content');
        
        const paragraph = document.createElement('p');
        paragraph.textContent = content;
        
        // Assemble message
        contentDiv.appendChild(paragraph);
        messageDiv.appendChild(contentDiv);
        
        // Add to chat
        chatMessages.appendChild(messageDiv);
        
        // Scroll to bottom
        chatMessages.scrollTop = chatMessages.scrollHeight;
        
        // Add to history
        chatHistory.push({ role, content });
        
        // Limit history length
        if (chatHistory.length > 10) {
            chatHistory.shift();
        }
    }
    
    /**
     * Process a user message and generate a response
     * @param {string} message - The user's message
     */
    async function processMessage(message) {
        // Set status to processing
        setStatus('processing');
        
        try {
            // For now, we'll implement a simple pattern matching system
            // In Phase 3, this will be replaced with actual LLM integration
            
            // Simple calculation pattern: "calculate X operator Y"
            const calcPattern = /calculate\s+(-?\d+\.?\d*)\s*([+\-*/×÷])\s*(-?\d+\.?\d*)/i;
            const calcMatch = message.match(calcPattern);
            
            if (calcMatch) {
                const num1 = parseFloat(calcMatch[1]);
                const operator = calcMatch[2];
                const num2 = parseFloat(calcMatch[3]);
                
                let result;
                
                // Perform calculation
                switch (operator) {
                    case '+':
                        result = num1 + num2;
                        break;
                    case '-':
                        result = num1 - num2;
                        break;
                    case '*':
                    case '×':
                        result = num1 * num2;
                        break;
                    case '/':
                    case '÷':
                        if (num2 === 0) {
                            addMessage("I can't divide by zero!", 'assistant');
                            setStatus('ready');
                            return;
                        }
                        result = num1 / num2;
                        break;
                    default:
                        addMessage("I don't understand that operator.", 'assistant');
                        setStatus('ready');
                        return;
                }
                
                // Format the result
                const formattedResult = Number.isInteger(result) ? result.toString() : result.toFixed(4).replace(/\.?0+$/, '');
                
                // Respond with the result
                addMessage(`The result of ${num1} ${operator} ${num2} is ${formattedResult}`, 'assistant');
                
                // Update calculator display if it's visible
                const display = document.getElementById('display');
                if (display && !display.closest('section').classList.contains('d-none')) {
                    display.textContent = formattedResult;
                }
            } 
            // Memory store pattern
            else if (/store\s+(-?\d+\.?\d*)\s+in\s+memory/i.test(message)) {
                const match = message.match(/store\s+(-?\d+\.?\d*)\s+in\s+memory/i);
                const value = parseFloat(match[1]);
                
                // Call WASM function
                calculatorFunctions.memory_store(value);
                
                addMessage(`I've stored ${value} in memory.`, 'assistant');
            }
            // Memory recall pattern
            else if (/recall\s+memory|what\'s\s+in\s+memory/i.test(message)) {
                // Call WASM function
                const value = calculatorFunctions.memory_recall();
                
                addMessage(`The value in memory is ${value}.`, 'assistant');
            }
            // Memory clear pattern
            else if (/clear\s+memory/i.test(message)) {
                // Call WASM function
                calculatorFunctions.memory_clear();
                
                addMessage(`I've cleared the memory.`, 'assistant');
            }
            // Help pattern
            else if (/help|what can you do/i.test(message)) {
                addMessage(
                    "I can help with calculations. Try asking me things like:\n" +
                    "- Calculate 125 × 37\n" +
                    "- Store 42 in memory\n" +
                    "- Recall memory\n" +
                    "- Clear memory\n" +
                    "In the future, I'll be able to handle more complex calculations and formulas!",
                    'assistant'
                );
            }
            // Default response
            else {
                addMessage(
                    "I'm not sure how to help with that yet. Try asking me to calculate something, like 'Calculate 125 × 37'.",
                    'assistant'
                );
            }
        } catch (error) {
            console.error('Error processing message:', error);
            addMessage("Sorry, I encountered an error processing your request.", 'assistant');
        }
        
        // Set status back to ready
        setStatus('ready');
    }
    
    /**
     * Set the chat status
     * @param {string} status - The status ('ready', 'processing', or 'error')
     */
    function setStatus(status) {
        chatStatus.innerHTML = '';
        
        const statusIcon = document.createElement('i');
        const statusText = document.createElement('small');
        
        switch (status) {
            case 'ready':
                statusIcon.className = 'bi bi-check-circle';
                statusText.textContent = ' Ready';
                chatStatus.className = 'text-success';
                chatInput.disabled = false;
                break;
            case 'processing':
                statusIcon.className = 'bi bi-hourglass-split';
                statusText.textContent = ' Processing...';
                chatStatus.className = 'text-warning';
                chatInput.disabled = true;
                break;
            case 'error':
                statusIcon.className = 'bi bi-exclamation-circle';
                statusText.textContent = ' Error';
                chatStatus.className = 'text-danger';
                chatInput.disabled = false;
                break;
        }
        
        chatStatus.appendChild(statusIcon);
        chatStatus.appendChild(statusText);
    }
    
    // Handle form submission
    chatForm.addEventListener('submit', (e) => {
        e.preventDefault();
        
        const message = chatInput.value.trim();
        
        if (message) {
            // Add user message to chat
            addMessage(message, 'user');
            
            // Clear input
            chatInput.value = '';
            
            // Process message
            processMessage(message);
        }
    });
    
    // Add focus and click handlers to ensure input works properly
    chatInput.addEventListener('focus', (e) => {
        // Ensure no other event handlers interfere with input
        e.stopPropagation();
    });
    
    chatInput.addEventListener('click', (e) => {
        // Ensure no other event handlers interfere with input
        e.stopPropagation();
    });
    
    chatInput.addEventListener('keydown', (e) => {
        // Ensure no other event handlers interfere with input
        e.stopPropagation();
    });
    
    // Set initial status
    setStatus('ready');
} 