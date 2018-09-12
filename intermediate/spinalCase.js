// Spinal Tap Case
// Convert a string to spinal case. Spinal case is all-lowercase-words-joined-by-dashes.


const spinalCase = (str) => str.split(/\W|[A-Z]/).join('-').toLowerCase();
  
 console.log(spinalCase('This Is Spinal Tap'));
 console.log(spinalCase("AllThe-small Things"));