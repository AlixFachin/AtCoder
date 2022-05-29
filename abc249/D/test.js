
const test_cases = [
    {
        input: ['3', '6 2 3'],
        output: ['2']
    },
    {
        input: ['1', '2'],
        output: ['0']
    },
    {
        input: ['10', '1 3 2 4 6 8 2 2 3 7'],
        output: ['62']
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
