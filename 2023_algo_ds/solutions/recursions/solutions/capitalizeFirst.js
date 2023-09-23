
// capitalizeFirst
// Write a recursive function called capitalizeFirst. Given an array of strings, capitalize the first letter of each string in the array.


function capitalizeFirst (array) {
  if (array.length === 1) {
    return [array[0][0].toUpperCase() + array[0].substr(1)];
  }
  const res = capitalizeFirst(array.slice(0, -1));
  const string = array.slice(array.length - 1)[0][0].toUpperCase() + array.slice(array.length-1)[0].substr(1);
  res.push(string);
  return res;
}
console.log(capitalizeFirst(['car','taco','banana'])); // ['Car','Taco','Banana']



const toUpperCase =(word) =>  word[0].toUpperCase() + word.substr(1);

const capitalizeLastWord = (words) =>  toUpperCase(words.slice(words.length - 1)[0]);

function capitalizeFirst2(words){
  if (words.length === 1) return [capitalizeLastWord(words)];
  // console.log("words.slice(0, -1)", words.slice(0, -1));
  const res = capitalizeFirst2(words.slice(0, -1));
  
  // console.log("AFTER...words.slice(0, -1)", words.slice(0, -1));
  // console.log("RES", res);
  const lastWordCapped = capitalizeLastWord(words);
  // console.log("lastWordCapped", lastWordCapped);
  res.push(lastWordCapped);
  return res;
}

console.log(capitalizeFirst2(['car','taco','banana'])); // ['Car','Taco','Banana']
// console.log("rere".slice(-2))
// console.log("first"[0].substring(2))

// console.log(['car','taco','banana'].slice(2)[0].substring(1))