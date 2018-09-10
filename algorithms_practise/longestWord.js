// Return the length of the longest word in the provided sentence.Your response
// should be a number.

const longestWord = (sentence) =>sentence.split(' ').sort((a,b)=> b.length-a.length)[0].length;

console.log(longestWord('Return the length of the longest word in the provided sentence'));
