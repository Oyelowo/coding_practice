// Mine

function twoNumberSum(array: number[], targetSum: number) {
  // Write your code here.

  return array.filter((el, i) =>
    array.some((el2, i2) => i !== i2 && el2 + el === targetSum)
  );
}

const arr = [3, 5, -4, 8, 11, 1, -1, 6];
const tar = 10;

console.log(twoNumberSum(arr, tar));

// Solution 2

function twoNumberSum(array: number[], targetSum: number) {
  // Write your code here.
  return array.filter(
    (el) =>
      !(el * 2 === targetSum) && array.some((el2) => el2 + el === targetSum)
  );
}

function twoNumberSum2(array: number[], targetSum: number) {
  return array.reduce((acc, val) => {
    const num =
      val * 2 === targetSum
        ? undefined
        : array.find((el) => el + val === targetSum);
    if (num) acc.push(num);
    return acc;
  }, []);
}

console.log("rearer", twoNumberSum2(arr, tar));

// Others

export function twoNumberSum3(array: number[], targetSum: number) {
  return array.reduce<number[]>((acc, val) => {
    if (acc.length === 2) {
      return acc;
    }
    if (val * 2 === targetSum) {
      return acc;
    }
    const num = array.find((el) => el + val === targetSum);
    if (num !== undefined) {
      acc.push(num);
    }
    return acc;
  }, []);
}


export function twoNumberSum(array: number[], targetSum: number) {
  // Write your code here.
  const sortedArray = array.sort((a, b) => a - b);
  let left = 0;
  let right = array.length - 1;

  while (left < right) {
    const currentSum = array[left] + array[right];
    if (currentSum === targetSum) {
      return [array[left], array[right]];
    } else if (currentSum < targetSum) {
      left++;
    } else if (currentSum > targetSum) {
      right--;
    }
  }

  return [];
}
