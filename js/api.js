/**
 * API module for the Rust WASM Calculator
 * Handles communication with the Anthropic API for LLM integration
 */

/**
 * Load the API key from the .env file
 * @returns {Promise<string>} The API key
 */
async function loadApiKey() {
    try {
        const response = await fetch('/.env');
        const text = await response.text();
        const match = text.match(/ANTHROPIC_API_KEY=([^\s]+)/);
        
        if (match && match[1]) {
            return match[1];
        } else {
            throw new Error('API key not found in .env file');
        }
    } catch (error) {
        console.error('Error loading API key:', error);
        throw error;
    }
}

/**
 * Send a message to the LLM through our server proxy
 * @param {Array} messages - Array of message objects with role and content
 * @param {Array} tools - Array of tool definitions
 * @returns {Promise<Object>} The API response
 */
export async function sendToLLM(messages, tools) {
    try {
        const response = await fetch('/api/llm', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                messages: messages,
                tools: tools
            })
        });
        
        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`API request failed: ${response.status} ${errorText}`);
        }
        
        return await response.json();
    } catch (error) {
        console.error('Error sending to LLM:', error);
        throw error;
    }
}

/**
 * Define calculator tools for the LLM
 * @returns {Array} Array of tool definitions
 */
export function defineCalculatorTools() {
    return [
        {
            name: 'calculate',
            description: 'Perform a calculation with two numbers',
            input_schema: {
                type: 'object',
                properties: {
                    num1: {
                        type: 'number',
                        description: 'The first number in the calculation'
                    },
                    num2: {
                        type: 'number',
                        description: 'The second number in the calculation'
                    },
                    operation: {
                        type: 'string',
                        enum: ['add', 'subtract', 'multiply', 'divide'],
                        description: 'The operation to perform'
                    }
                },
                required: ['num1', 'num2', 'operation']
            }
        },
        {
            name: 'memory_store',
            description: 'Store a value in calculator memory',
            input_schema: {
                type: 'object',
                properties: {
                    value: {
                        type: 'number',
                        description: 'The value to store in memory'
                    }
                },
                required: ['value']
            }
        },
        {
            name: 'memory_recall',
            description: 'Recall the value from calculator memory',
            input_schema: {
                type: 'object',
                properties: {}
            }
        },
        {
            name: 'memory_clear',
            description: 'Clear the calculator memory',
            input_schema: {
                type: 'object',
                properties: {}
            }
        }
    ];
}

/**
 * Parse the LLM response for function calls
 * @param {Object} response - The API response
 * @returns {Array} Array of function call objects
 */
export function parseFunctionCalls(response) {
    const functionCalls = [];
    
    if (response.content && Array.isArray(response.content)) {
        for (const content of response.content) {
            if (content.type === 'tool_use') {
                functionCalls.push({
                    name: content.name,
                    arguments: content.input
                });
            }
        }
    }
    
    return functionCalls;
} 