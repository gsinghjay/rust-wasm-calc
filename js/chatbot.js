/**
 * Chatbot module for the Rust WASM Calculator
 * Handles chat interface and integration with calculator functions
 */

import { sendToLLM, defineCalculatorTools, parseFunctionCalls } from './api.js';

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
    
    // Define calculator tools for the LLM
    const tools = defineCalculatorTools();
    
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
            // Format messages for the API
            const messages = chatHistory.map(msg => ({
                role: msg.role,
                content: msg.content
            }));
            
            // Send to LLM
            const response = await sendToLLM(messages, tools);
            
            // Extract assistant message
            let assistantMessage = '';
            if (response.content && Array.isArray(response.content)) {
                for (const content of response.content) {
                    if (content.type === 'text') {
                        assistantMessage += content.text;
                    }
                }
            }
            
            // Add assistant message to chat
            if (assistantMessage) {
                addMessage(assistantMessage, 'assistant');
            }
            
            // Handle function calls
            const functionCalls = parseFunctionCalls(response);
            
            for (const call of functionCalls) {
                await handleFunctionCall(call);
            }
        } catch (error) {
            console.error('Error processing message:', error);
            addMessage("Sorry, I encountered an error processing your request.", 'assistant');
        }
        
        // Set status back to ready
        setStatus('ready');
    }
    
    /**
     * Handle a function call from the LLM
     * @param {Object} functionCall - The function call object
     */
    async function handleFunctionCall(functionCall) {
        const { name, arguments: args } = functionCall;
        
        try {
            let result;
            
            switch (name) {
                case 'calculate':
                    const { num1, num2, operation } = args;
                    
                    switch (operation) {
                        case 'add':
                            result = num1 + num2;
                            break;
                        case 'subtract':
                            result = num1 - num2;
                            break;
                        case 'multiply':
                            result = num1 * num2;
                            break;
                        case 'divide':
                            if (num2 === 0) {
                                addMessage("I can't divide by zero!", 'assistant');
                                return;
                            }
                            result = num1 / num2;
                            break;
                    }
                    
                    // Format the result
                    const formattedResult = Number.isInteger(result) ? result.toString() : result.toFixed(4).replace(/\.?0+$/, '');
                    
                    // Update calculator display if it's visible
                    const display = document.getElementById('display');
                    if (display && !display.closest('section').classList.contains('d-none')) {
                        display.textContent = formattedResult;
                    }
                    
                    addMessage(`The result of ${operation}(${num1}, ${num2}) is ${formattedResult}`, 'assistant');
                    break;
                    
                case 'memory_store':
                    calculatorFunctions.memory_store(args.value);
                    addMessage(`I've stored ${args.value} in memory.`, 'assistant');
                    break;
                    
                case 'memory_recall':
                    result = calculatorFunctions.memory_recall();
                    addMessage(`The value in memory is ${result}.`, 'assistant');
                    break;
                    
                case 'memory_clear':
                    calculatorFunctions.memory_clear();
                    addMessage(`I've cleared the memory.`, 'assistant');
                    break;
            }
        } catch (error) {
            console.error(`Error executing function ${name}:`, error);
            addMessage(`Sorry, I encountered an error executing the ${name} function.`, 'assistant');
        }
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