const { readFileSync } = require('fs');

let DEBUG = true;

// -=-=-=-=-= Beginning of task-relative stuff
// ABC 246 - B - Get Closer

function main(inputString) {
    const [ a, b ] = inputString.split(' ');
    if (DEBUG) console.log(`Using a:${a} and b:${b}`);

    const x = a/(Math.sqrt(a**2+b**2));
    const y = b/(Math.sqrt(a**2+b**2));
    return [`${x} ${y}`];

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