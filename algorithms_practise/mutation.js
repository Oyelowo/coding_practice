// Return true if the string in the first element of the array contains all of
// the letters of the string in the second element of the array.For example, [
//   "hello", "Hello" ], should return true because all of the letters in the
// second string are present in the first, ignoring case.The arguments["hello",
//    "hey"]should return false because the string "hello" does not contain a
// "y".Lastly, [     "Alien", "line" ], should return true because all of the
// letters in "line" are present in "Alien".

const mutation = (arr) => {
    let [firstElement, secondElement]= arr;
    firstElement = firstElement.toLowerCase().split('');
    secondElement = secondElement.toLowerCase().split('');
   return secondElement.every(second=> firstElement.some(first=> first===second));
};

// longerSolution
// 
// const mutation = (arr) => {
//     let [firstElement, secondElement]= arr;
//     firstElement = firstElement.toLowerCase().split('');
//     secondElement = secondElement.toLowerCase().split('');
//     let array=[];
//   for(let item of secondElement) {
//      array=array.concat(firstElement.some(first=>first===item));
//   }
//   if (array.some(el=> el===false)) {
//       return false;
//   };
//   return true 
// };

 console.log(mutation(["hello", "hlle"]));
 console.log(mutation(["zyxwvutsrqponmlkjihgfedcba", "qrstu"]));
 console.log(mutation(["Mary", "Aarmy"]));