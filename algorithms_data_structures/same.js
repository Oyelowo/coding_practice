"use strict";
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
var countNumbersInArraytoObj = function (array) {
    var obj = {};
    array.forEach(function (el) {
        if (obj.hasOwnProperty(el)) {
            obj[el] += 1;
        }
        else {
            obj[el] = 1;
        }
    });
    return obj;
};
var same = function (array1, array2) {
    var array1Obj = countNumbersInArraytoObj(array1);
    var array2Obj = countNumbersInArraytoObj(array2);
    for (var key in array1Obj) {
        var doubleNum = Math.pow(parseInt(key), 2);
        if (array2Obj[doubleNum] === array1Obj[key]) {
            delete array2Obj[doubleNum];
        }
    }
    return Object.keys(array2Obj).length === 0 && array2Obj.constructor === Object;
};
console.log(same([1, 2, 3, 2], [9, 1, 4, 4]));
//console.log(countNumbersInArraytoObj([3,4,5,3,4,5]))
