export function twoNumberSum(array, targetSum) {
    // Write your code here.

    return array.filter((el, i) => array.some((el2, i2) => (i !== i2) && ((el2 + el) === targetSum)))
}

const arr = [3, 5, -4, 8, 11, 1, -1, 6];
const tar = 10;

console.log(twoNumberSum(arr, tar));



// Solution 2

export function twoNumberSum(array: number[], targetSum: number) {
    // Write your code here.
    return array.filter((el) => !((el * 2) === targetSum) && array.some((el2) => ((el2 + el) === targetSum)));
}

