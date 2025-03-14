/**
 * Test runner for JavaScript tests
 */

const fs = require('fs');
const path = require('path');
const { exec } = require('child_process');

// Define test directories
const testDirs = ['tests'];

// Find all test files
function findTestFiles() {
    const testFiles = [];
    
    for (const dir of testDirs) {
        if (fs.existsSync(dir)) {
            const files = fs.readdirSync(dir);
            
            for (const file of files) {
                if (file.endsWith('_tests.js')) {
                    testFiles.push(path.join(dir, file));
                }
            }
        }
    }
    
    return testFiles;
}

// Run a test file
function runTest(file) {
    return new Promise((resolve, reject) => {
        console.log(`\nRunning test: ${file}`);
        console.log('='.repeat(50));
        
        // Use Node.js to run the test
        exec(`node ${file}`, (error, stdout, stderr) => {
            if (error) {
                console.error(`Error running test ${file}:`);
                console.error(stderr);
                reject(error);
                return;
            }
            
            console.log(stdout);
            resolve();
        });
    });
}

// Run all tests
async function runAllTests() {
    const testFiles = findTestFiles();
    
    if (testFiles.length === 0) {
        console.log('No test files found.');
        return;
    }
    
    console.log(`Found ${testFiles.length} test files.`);
    
    let passedCount = 0;
    let failedCount = 0;
    
    for (const file of testFiles) {
        try {
            await runTest(file);
            passedCount++;
        } catch (error) {
            failedCount++;
        }
    }
    
    console.log('\nTest Summary:');
    console.log('='.repeat(50));
    console.log(`Total: ${testFiles.length}`);
    console.log(`Passed: ${passedCount}`);
    console.log(`Failed: ${failedCount}`);
    
    if (failedCount > 0) {
        process.exit(1);
    }
}

// Run the tests
runAllTests().catch(error => {
    console.error('Error running tests:', error);
    process.exit(1);
}); 