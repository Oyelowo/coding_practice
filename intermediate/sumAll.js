// We'll pass you an array of two numbers. Return the sum of those two numbers
// plus the sum of all the numbers between them. The lowest number will not
// always come first.

const sumAll = (arr) => {
    let smallerNumber = Math.min(...arr);
    let largerNumber = Math.max(...arr);
    let number = smallerNumber;
    while (number <= largerNumber) {
        number++;
    }
    return number;
};