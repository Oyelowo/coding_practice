// Check if a value is classified as a boolean primitive. Return true or false.
// Boolean primitives are true and false.

const booWho = (bool) =>bool===true || bool===false;
 
console.log( booWho(true));
console.log( booWho('true'));
console.log( booWho(false));
console.log( booWho('false'));
console.log(booWho([].slice));
console.log(booWho(NaN)); 
