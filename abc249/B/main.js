const { readFileSync } = require('fs');

let DEBUG = true;

// -=-=-=-=-= Beginning of task-relative stuff

function isUpperCase(x) {
    return x === x.toUpperCase();
}

function isLowerCase(x) {
    return x === x.toLowerCase();
}

function main(inputString) {

    const lineSep = inputString.split('\n');
    const myString = lineSep[0];

    if (myString.length > 52 || myString.length < 2) {
        return ['No'];
    }

    const hashObj = {};
    let hasUp = false;
    let hasDown = false; 

    for (let x of myString) {
        if (isUpperCase(x)) {
            hasUp = true;
        }
        if (isLowerCase(x)) {
            hasDown = true;
        }
        if (!hashObj[x]) {
            hashObj[x] = 1;
        } else {
            hashObj[x] = hashObj[x] + 1;
        }
    }
    
    if (DEBUG) { console.log(JSON.stringify(hashObj)) }
    for (let x of Object.keys(hashObj)) {
        if (hashObj[x] > 1) {
            return ['No'];
        }
    }


    return ((hasUp && hasDown) ? ['Yes'] : ['No']);

}
// -=-=-=-=-=-=-=-= End of task-dependant code

if (require.main !== module) {
    // UNIT test_cases

    if (!isLowerCase('z')) console.error('Error 1 in LowerCase');
    if (isLowerCase('Z')) console.error('Error 2 in LowerCase');
    if (!isUpperCase('Z')) console.error('Error 1 in UpperCase');
    if (isUpperCase('z')) console.error('Error 2 in UpperCase');

}

if (require.main === module) {
    // main launch of the file during the contest
    DEBUG = false;
    const result = main(readFileSync('/dev/stdin', 'utf-8'));
    for (let i=0; i< result.length; i++) {
        console.log(result[i]);
    }
}

exports.main = main;