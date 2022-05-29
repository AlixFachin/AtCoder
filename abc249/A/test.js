
const test_cases = [
    /* Structure: 
        input = Array of strings. One string = one STDIN line (space separated)
        output = likewise (usually only one line though)
    {
        input: ['-1 -1', '-1 2', '3 2'],
        output: ['3 -1']
    }, 
    */
    {
        input: ['4 3 3 6 2 5 10'],
        output: ['Takahashi'],
    },
    {
        input: ['3 1 4 1 5 9 2'],
        output: ['Aoki'],
    },
   {
    input: ['1 1 1 1 1 1 1'],
    output: ['Draw'],
}
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
