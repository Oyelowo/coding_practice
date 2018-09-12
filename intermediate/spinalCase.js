// Spinal Tap Case Convert a string to spinal case. Spinal case is
// all-lowercase-words-joined-by-dashes.

const isUpperCase = (letter) => letter === letter.toUpperCase() && !typeof(letter) === 'number';

// console.log(isUpperCase('g'));

const spinalCase = (str) => {
    for(let item of str) {
      if(isUpperCase(item)){
          
      }
    }
    
   return str.split(/\W/).join('-')
};

console.log(spinalCase('This Is Spinal Tap'));
console.log(spinalCase("AllThe-small Things"));