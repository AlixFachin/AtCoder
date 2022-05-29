const { readFileSync } = require('fs');

let DEBUG = true;

// -=-=-=-=-= Beginning of task-relative stuff

function main(inputString) {
    const [a, b, k] = inputString.split(' ');
    if (DEBUG) console.log(`Array input: ${[a, b, k]}`);
    
    if (a >= b) {
        return ['0']; // already enough from the start
    }

    const logresult = Math.ceil((Math.log(b) - Math.log(a))/Math.log(k));
    
    if (DEBUG) console.log(`Result: ${logresult}`);


    return [`${logresult}`]

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