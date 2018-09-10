
// Repeat a given string str(first argument)for num times(second
// argument).Return an empty string if num is not a positive number.
const repeatStringNumTimes = (str, num) => {
    let i=0;
    let word='';
    while (i<num) {
        word+=str;
        i++;
    }
    return word;
  }
  
  console.log(repeatStringNumTimes('love', 55));
  
  