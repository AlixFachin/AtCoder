const { readFileSync } = require('fs');

let DEBUG = true;

// -=-=-=-=-= Beginning of task-relative stuff


function buildString(n) {
    if (n == 1) {
        return '1'
    }
    const prev = buildString(n-1);
    return `${prev} ${n} ${prev}`;
}

function main(inputString) {

    if (DEBUG) console.log(`Receiving ${inputString}`);
    const lineSeparated = inputString.split('\n');
    const result = [];
    for (let line of lineSeparated) {
        result.push(line.split(' '));
    }
    if (DEBUG) console.log(`Parsed into ${JSON.stringify(result)}`);
    
    const [[N]] = result;
    return([ buildString(N) ]);

}
// -=-=-=-=-=-=-=-= End of task-dependant code

if (require.main === module) {
    // main launch of the file during the contest
    DEBUG = false;
    const result = main(readFileSync('/dev/stdin', 'utf-8'));
    for (let i=0; i< result.length; i++) {
        console.log(result[i]);
    }
}

exports.main = main;