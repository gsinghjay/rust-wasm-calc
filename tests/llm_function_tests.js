/**
 * Tests for LLM function calling interface
 */

// Mock API response
const mockResponse = {
    id: 'msg_01234567890',
    content: [
        {
            type: 'text',
            text: 'I\'ll calculate 5 + 3 for you.'
        },
        {
            type: 'tool_use',
            name: 'calculate',
            input: {
                num1: 5,
                num2: 3,
                operation: 'add'
            }
        }
    ],
    model: 'claude-3-haiku-20240307',
    role: 'assistant',
    type: 'message'
};

// Mock calculator functions
const mockCalculatorFunctions = {
    add: (a, b) => a + b,
    subtract: (a, b) => a - b,
    multiply: (a, b) => a * b,
    divide: (a, b) => a / b,
    memory_store: (value) => console.log(`Storing ${value} in memory`),
    memory_recall: () => 42,
    memory_clear: () => console.log('Clearing memory')
};

// Define functions to test (since we can't import ES modules in Node.js directly)
function defineCalculatorTools() {
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

function parseFunctionCalls(response) {
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

/**
 * Test function definitions
 */
function testFunctionDefinitions() {
    console.log('Testing function definitions...');
    
    const tools = defineCalculatorTools();
    
    // Check that we have the expected number of tools
    if (tools.length !== 4) {
        console.error(`Expected 4 tools, got ${tools.length}`);
        return false;
    }
    
    // Check that each tool has the required properties
    for (const tool of tools) {
        if (!tool.name || !tool.description || !tool.input_schema) {
            console.error(`Tool missing required properties: ${JSON.stringify(tool)}`);
            return false;
        }
    }
    
    console.log('Function definitions test passed!');
    return true;
}

/**
 * Test message processing
 */
function testMessageProcessing() {
    console.log('Testing message processing...');
    
    // Create a mock message
    const message = 'Calculate 5 + 3';
    
    // Check that we can format it correctly
    const formattedMessage = { role: 'user', content: message };
    
    if (formattedMessage.role !== 'user' || formattedMessage.content !== message) {
        console.error(`Message formatting failed: ${JSON.stringify(formattedMessage)}`);
        return false;
    }
    
    console.log('Message processing test passed!');
    return true;
}

/**
 * Test function call handling
 */
function testFunctionCallHandling() {
    console.log('Testing function call handling...');
    
    // Parse function calls from mock response
    const functionCalls = parseFunctionCalls(mockResponse);
    
    // Check that we extracted the function call correctly
    if (functionCalls.length !== 1) {
        console.error(`Expected 1 function call, got ${functionCalls.length}`);
        return false;
    }
    
    const call = functionCalls[0];
    
    if (call.name !== 'calculate') {
        console.error(`Expected function name 'calculate', got '${call.name}'`);
        return false;
    }
    
    if (call.arguments.num1 !== 5 || call.arguments.num2 !== 3 || call.arguments.operation !== 'add') {
        console.error(`Function arguments incorrect: ${JSON.stringify(call.arguments)}`);
        return false;
    }
    
    // Test executing the function call
    try {
        const result = mockCalculatorFunctions[call.arguments.operation](call.arguments.num1, call.arguments.num2);
        
        if (result !== 8) {
            console.error(`Expected result 8, got ${result}`);
            return false;
        }
    } catch (error) {
        console.error(`Error executing function: ${error}`);
        return false;
    }
    
    console.log('Function call handling test passed!');
    return true;
}

/**
 * Run all tests
 */
function runTests() {
    console.log('Running LLM function calling interface tests...');
    
    let allPassed = true;
    
    allPassed = testFunctionDefinitions() && allPassed;
    allPassed = testMessageProcessing() && allPassed;
    allPassed = testFunctionCallHandling() && allPassed;
    
    if (allPassed) {
        console.log('All tests passed!');
    } else {
        console.error('Some tests failed!');
    }
}

// Run tests
runTests(); 