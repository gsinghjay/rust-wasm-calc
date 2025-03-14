/**
 * Server for the Rust WASM Calculator
 * Handles API requests to Anthropic securely
 */

const express = require('express');
const cors = require('cors');
const dotenv = require('dotenv');
const path = require('path');
const fetch = require('node-fetch');

// Load environment variables
dotenv.config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(cors());
app.use(express.json());
app.use(express.static(path.join(__dirname)));

// API proxy endpoint
app.post('/api/llm', async (req, res) => {
    try {
        const { messages, tools } = req.body;
        
        // Get API key from environment
        const apiKey = process.env.ANTHROPIC_API_KEY;
        
        if (!apiKey) {
            return res.status(500).json({ error: 'API key not configured' });
        }
        
        // Forward request to Anthropic
        const response = await fetch('https://api.anthropic.com/v1/messages', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'x-api-key': apiKey,
                'anthropic-version': '2023-06-01'
            },
            body: JSON.stringify({
                model: 'claude-3-haiku-20240307',
                max_tokens: 1024,
                messages,
                tools
            })
        });
        
        if (!response.ok) {
            const errorText = await response.text();
            return res.status(response.status).json({ error: `API request failed: ${errorText}` });
        }
        
        const data = await response.json();
        return res.json(data);
    } catch (error) {
        console.error('Error in API proxy:', error);
        return res.status(500).json({ error: error.message });
    }
});

// Start server
app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
}); 