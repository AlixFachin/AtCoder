const { readFileSync } = require('fs');

let DEBUG = true;

// -=-=-=-=-= Beginning of task-relative stuff

function main(inputString) {
    let numbArray = inputString.split('');
    if (DEBUG) console.log(numbArray);

    const resultHash = {};
    for (let i=0; i<9; i++ ) {
        resultHash[numbArray[i]] = true;
    }
    for (let i=0; i<10; i++) {
        if (!resultHash[i]) {
            return [`${i}`];
        }
    }

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