
const test_cases = [
    {
        input: ['5 4 7','8 3 10 5 13'],
        output: ['12']
    },
    {
        input: ['5 100 7','8 3 10 5 13'],
        output: ['0']
    },
    {
        input: ['20 815 60','2066 3193 2325 4030 3725 1669 1969 763 1653 159 5311 5341 4671 2374 4513 285 810 742 2981 202'],
        output: ['112']
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
