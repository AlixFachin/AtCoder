const { readFileSync } = require('fs');

let DEBUG = true;

// -=-=-=-=-= Beginning of task-relative stuff

function main(inputString) {
    const lineSep = inputString.split('\n');
    const numArray = lineSep[1].split(' ').map(x => Number(x));

    const arrayDic = {};
    for (let i=0; i<numArray.length; i++) {
        arrayDic[String(numArray[i])] = (arrayDic[String(numArray[i])] ? arrayDic[String(numArray[i])] : 0) + 1;
    }
    let numTriplets = 0;
    for (let i = 0; i < numArray.length; i++) {
        if ( arrayDic[String(numArray[i] * numArray[i])] ) {
            numTriplets += arrayDic[String(numArray[i] * numArray[i])];
        }
        for (let j = i+1; j < numArray.length; j++) {
            if ( arrayDic[String(numArray[i] * numArray[j])] ) {
                numTriplets += 2*arrayDic[String(numArray[i] * numArray[j])];
            }
        }
    }

    return [String(numTriplets)];

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