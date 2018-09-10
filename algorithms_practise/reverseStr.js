// // Reverse the provided string. You may need to turn the string into an array //
// before you can reverse it. Your result must be a string. Remember to use //
// Read-Search-Ask if you get stuck. Write your own code.

const reverseString = str => str
.split('')
.reverse().join('');
console.log(reverseString('Oyelowo'));

// without a helper function
const reverseStr = (str) => {
let reversedString='';
for (let i = str.length-1; i >= 0; i--) {
    reversedString+=str[i];
};
return reversedString;
};
console.log(reverseStr('Dayo'));;

