const { readFileSync } = require('fs');

let DEBUG = true;

// -=-=-=-=-= Beginning of task-relative stuff


function main(inputString) {

    if (DEBUG) console.log(`Receiving ${inputString}`);
    const lineSeparated = inputString.split('\n');
    const result = [];
    for (let line of lineSeparated) {
        result.push(line.split(' '));
    }
    if (DEBUG) console.log(`Parsed into ${JSON.stringify(result)}`);
    
    const frequencyTable = {};

    const [ N ] = result[0];
    for (let i=0; i< N; i++) {
        const [ surname, firstname ] = result[i + 1];
        if (!frequencyTable[surname]) {
            frequencyTable[surname] = 1;
        } else {
            frequencyTable[surname] = frequencyTable[surname] + 1;
        }
        if (firstname != surname) {
            if (!frequencyTable[firstname]) {
                frequencyTable[firstname] = 1;
            } else {
                frequencyTable[firstname] = frequencyTable[firstname] + 1;
            }
        }
    }

    // second pass
    for (let i=0; i < N; i++) {
        const [ surname, firstname ] = result[i + 1];
        if (frequencyTable[surname] > 1 && frequencyTable[firstname] > 1) {
            // Then we can choose the surname 
            return ['No'];
        }
    }

    return([ 'Yes' ]);

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