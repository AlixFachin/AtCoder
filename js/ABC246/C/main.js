const { readFileSync } = require('fs');

let DEBUG = true;

// -=-=-=-=-= Beginning of task-relative stuff

function getMaximumIndex(a) {
    let curMax = 0;
    let curIndex = -1;
    for (let i=0; i< a.length; i++) {
        if (Number(a[i]) > curMax) {
            curMax = Number(a[i]);
            curIndex = i;
        }
    }
    return curIndex;
}

function main(inputString) {
    const lineSeparated = inputString.split('\n');
    const [n, k, x] = lineSeparated[0].split(' ');
    const nrCoupons = Number(k);
    const discount = Number(x);

    priceArray = lineSeparated[1].split(' ');

    let couponRemaining = nrCoupons;
    let currentSum = 0;
    let nrCouponUsed = 0; // on one item
    let maxIndex;
    if (DEBUG) console.log(`Array before loop: ${priceArray}, nrCoup: ${nrCouponUsed}`)
    while (couponRemaining > 0 && priceArray.length > 0) {
        // First, find the maximum
        maxIndex = getMaximumIndex(priceArray);
        if (priceArray[maxIndex] > discount) {
            // Still able to use full coupons!
            nrCouponUsed = Math.min(couponRemaining,  Math.floor(priceArray[maxIndex] / discount));
        } else {
            // Not able to use full-discount, gotta use what we can
            nrCouponUsed = 1;
        }
        if (DEBUG) console.log(`Using ${nrCouponUsed} for item ${maxIndex} of price ${priceArray[maxIndex]}`)
        const priceRemaining = Number(priceArray[maxIndex]) - (nrCouponUsed * discount);
        if (priceRemaining > 0) {
            priceArray[maxIndex] = priceRemaining;
        } else {
            priceArray.splice(maxIndex, 1);
        }
        couponRemaining = couponRemaining - nrCouponUsed;
        if (DEBUG) console.log(`Current array: ${priceArray}, current sum: ${currentSum}, remains ${couponRemaining}`)
    }
    if (DEBUG) console.log(`Array after loop: ${priceArray} `)
    currentSum += priceArray.reduce((prev, x) => (Number(x) + Number(prev)), 0);
    return( [`${currentSum}`]);
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