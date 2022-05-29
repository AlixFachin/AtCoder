const { readFileSync } = require('fs');

let DEBUG = true;

function log(outString) {
    if (DEBUG) console.log(outString);
}

// -=-=-=-=-= Beginning of task-relative stuff
// main will read the whole string, so in order to get the components
// need to do 
// const lineSeparated = inputString.split('\n');
// const 
function main(inputString) {
    const [ a, b, c, d, e, f, raceLength] = inputString.split(' ').map(s => Number(s));

    const runFunc = (speed, duration, rest) => {
        
        const getResult = x => {
            const xMod = x % (duration + rest);
            const xDiv = Math.floor( x / (duration + rest) );
            const step = duration*speed;
            if (xMod <= duration) {
                return xMod*speed + xDiv*step;
            } else {
                return duration*speed + xDiv*step;
            }
        }

        return getResult;
    }

    const Taka = runFunc(b, a, c);
    const Aoki = runFunc(e, d, f);
    
    if (require.main !== module) {
    
        for (let i=1; i < 20; i++) {
            console.log(`i=${i}, Taka=${Taka(i)}, Aoki=${Aoki(i)}`);
        }
    
    }

    if (Taka(raceLength) > Aoki(raceLength)) {
        return ['Takahashi'];
    } else if (Taka(raceLength) < Aoki(raceLength)) {
        return ['Aoki'];
    } else {
        return ['Draw'];
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