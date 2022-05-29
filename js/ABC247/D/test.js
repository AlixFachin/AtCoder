
const test_cases = [
    {
        input: ['4', '1 2 3', '2 2', '1 3 4', '2 5'],
        output: ['4', '14']
    },
    {
        input: ['2', '1 1000000000 1000000000', '2 1000000000'],
        output: ['1000000000000000000']
    },
    {
        input: ['5','1 1 1','1 1 1','1 1 1','1 1 1','1 1 1'],
        output: []
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
const DEBUG = true;
process.env.DEBUG = true;
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
