// Return the factorial of the provided integer. If the integer is represented
// with the letter n, a factorial is the product of all positive integers less
// than or equal to n. Factorials are often represented with the shorthand
// notation n! For example: 5! = 1 * 2 * 3 * 4 * 5 = 120 Only integers greater
// than or equal to zero will be supplied to the function.// Return the factorial of the provided integer. If the integer is represented
// with the letter n, a factorial is the product of all positive integers less
// than or equal to n. Factorials are often represented with the shorthand
// notation n! For example: 5! = 1 * 2 * 3 * 4 * 5 = 120 Only integers greater
// than or equal to zero will be supplied to the function.

const factorial = (num) => {
    if (num <= 0 || isNaN(num)) {
        throw new Error('Has to be a number and be greater than zero');
    }
    let i = 1;
    let factorialisedNum = 1;
    while (i <= num) {
        factorialisedNum *= i;
        i++;
    }
    return factorialisedNum;
};

console.log(factorial());
