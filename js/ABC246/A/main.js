const { readFileSync } = require('fs');

let DEBUG = true;
// -=-=-=-=-= Beginning of task-relative stuff
// ABC246 - "Four Points"
function findUnique(valArray) {
    
    const freq = {};
    for (let x of valArray) {
        if (freq[x]) {
            freq[x]++;
        } else {
            freq[x] = 1;
        }
    }
    for (let x in freq) {
        if (freq[x] == 1) {
            return x;
        }
    }
}

function main(inputString) {
    if (DEBUG) console.log(`Receiving ${inputString}`);
    const lineSeparated = inputString.split('\n');
    const result = [];
    for (let line of lineSeparated) {
        result.push(line.split(' '));
    }
    if (DEBUG) console.log(`Parsed into ${JSON.stringify(result)}`);
    
    const [ [x1, y1], [x2, y2], [x3, y3] ] = result;

    const x4 = findUnique([x1, x2, x3]);
    const y4 = findUnique([y1, y2, y3]);

    return([x4 + ' ' + y4]);

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