
const test_cases = [
    {
        input: ['AtCoder'],
        output: ['Yes']
    },
    {
        input: ['Aa'],
        output: ['Yes']
    },
    {
        input: ['atcoder'],
        output: ['No']
    },
    {
        input: ['Perfect'],
        output: ['No']
    },
    {
        input: ['ATCODER'],
        output: ['No']
    },
    {
        input: ['ATCODERAAAAA'],
        output: ['No']
    },
    {
        input: ['A'],
        output: ['No']
    },
    {
        input: ['a'],
        output: ['No']
    },
    {
        input: ['ABCDEFGHIJKLMNOPQRTSUVWXYZabcdefghijklmnopqrstuvw'],
        output: ['Yes']
    },

];

const { main } = require('./main.js');

function array_equal(a, b) {
    
    if (!(a instanceof Array)) {
        return false;
    }
    if (!(b instanceof Array)) {
        return false;
    }
    
    if (a.length !== b.length) { 
        return false;
    }
    
    for (let i=0; i< a.length; i++) {
        if (a[i] !== b[i]) {
            return false;
        }
    }
    
    return true;
}

// MAIN TEST LOOP
for (let i=0; i < test_cases.length; i++) {
    const currentTest = test_cases[i];
    console.log(`Running test ${i}, ${JSON.stringify(currentTest)}`)
    const result = main(currentTest.input.join('\n'));
    if (array_equal(result, currentTest.output) ) {
        console.log(`Test ${i} PASSED!`)
    } else {
        console.log(`Test ${i} FAILED!!!`);
        console.log(`Expected: ${JSON.stringify(currentTest.output)}`);
        console.log(`Received: ${JSON.stringify(result)}`);
        break;
    }
}
