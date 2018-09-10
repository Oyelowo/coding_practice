// Truncate a string (first argument) if it is longer than the given maximum
// string length (second argument). Return the truncated string with a ...
// ending.

const truncateString = (str, num) => str.length >num ?`${str.substring(0, num + 1)}...`: str;

console.log( truncateString("A-tisket a-tasket A green and yellow basket", 8));
console.log(truncateString("A-tisket a-tasket A green and yellow basket", "A-tisket a-tasket A green and yellow basket".length + 2));
console.log(truncateString("A-", 1));