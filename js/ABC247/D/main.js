const { readFileSync } = require('fs');

let DEBUG = true;

// -=-=-=-=-= Beginning of task-relative stuff
let pipe = [];

function removeValueAndReturnSum(toRemove) {
    if (DEBUG) console.log(`Trying to remove ${toRemove} from ${JSON.stringify(pipe)}`);
    if (toRemove == 0) {
        return 0;
    }
    
    let currentRemove = toRemove;
    let result = 0;
    while (currentRemove > 0) {
        const firstBalls = pipe.shift();
    
        if (currentRemove < firstBalls.qty) {
            if (DEBUG) console.log(`(${firstBalls.qty}-${currentRemove})Low quantity ${JSON.stringify(pipe)}`);
            pipe.unshift({
                value: firstBalls.value,
                qty: firstBalls.qty - currentRemove,
            })
            if (DEBUG) console.log(`Low quantity ${JSON.stringify(pipe)}`);
            result += (firstBalls.value * currentRemove); 
            currentRemove = 0;
        } else {
            result += firstBalls.value * firstBalls.qty;
            currentRemove = currentRemove - firstBalls.qty;
        }
    }
    return result;

}

function main(inputString) {

    if (DEBUG) console.log(`Receiving ${inputString}`);
    const lineSeparated = inputString.split('\n');
    const result = [];
    for (let line of lineSeparated) {
        result.push(line.split(' '));
    }
    pipe = [];
    let outputArray = [];
    const [Q] = result[0];
    for (let i=0; i < Q; i++) {
        const operation = result[i + 1];
        if (DEBUG) console.log(`${i} : ${operation}`);
        if (operation[0] == 1) {
            pipe.push({value: Number(operation[1]), qty: Number(operation[2])});            
            if (DEBUG) console.log(`After op: ${JSON.stringify(pipe)}`);
        } else if (operation[0] == 2) {
            const curSum = removeValueAndReturnSum(Number(operation[1]));
            
            outputArray.push(String(curSum));
            if (DEBUG) console.log(`After shift: ${JSON.stringify(pipe)}, result: ${outputArray}`);
        }
    }
    return outputArray;

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