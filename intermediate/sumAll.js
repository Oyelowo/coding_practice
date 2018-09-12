// We'll pass you an array of two numbers. Return the sum of those two numbers
// plus the sum of all the numbers between them. The lowest number will not
// always come first.

const sumAll = (arr) => {
    let [smallerNumber, largerNumber] = [Math.min(...arr), Math.max(...arr)];
    let number = 0
    while (smallerNumber <= largerNumber) {
        number += smallerNumber;
        smallerNumber++;
    }
    return number;
};
console.log(sumAll([1, 4]));