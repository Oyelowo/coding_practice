/* const countItemInArray = (array: Array<number>, itemToCount: number) =>
  array.reduce(
    (accumulator, val) => accumulator + Number(val === itemToCount),
    0
  );

const same = (arr1: Array<number>, arr2: Array<number>): any => {
  const arr1Squared = arr1.map(el => el ** 2);
  const wetin = arr1Squared.every(
    el => countItemInArray(arr1Squared, el) === countItemInArray(arr2, el)
  );

  return wetin;
};
 */


 
const countNumbersInArraytoObj = (array: Array<number>): any => {
  const obj : any = {};
  array.forEach(el => {
    if (obj.hasOwnProperty(el)) {
      obj[el] += 1;
    } else {
      obj[el] = 1;
    }
  });
  return obj;
};

const same = (array1: number[], array2: Array<number>): boolean => {
  const array1Obj = countNumbersInArraytoObj(array1);
  const array2Obj = countNumbersInArraytoObj(array2);
  for (const key in array1Obj) {
    const doubleNum = parseInt(key) ** 2;
    if (array2Obj[doubleNum] === array1Obj[key]) {
      delete array2Obj[doubleNum];
    }

  }
  return Object.keys(array2Obj).length === 0 && array2Obj.constructor === Object;
};

console.log(same([1, 2, 3, 2], [9, 1, 4, 4]));
//console.log(countNumbersInArraytoObj([3,4,5,3,4,5]))
