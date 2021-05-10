// Mine

function twoNumberSum(array, targetSum) {
  // Write your code here.

  return array.filter((el, i) =>
    array.some((el2, i2) => i !== i2 && el2 + el === targetSum)
  );
}

const arr = [3, 5, -4, 8, 11, 1, -1, 6];
const tar = 10;

console.log(twoNumberSum(arr, tar));

// Solution 2

function twoNumberSum(array, targetSum) {
  // Write your code here.
  return array.filter(
    (el) =>
      !(el * 2 === targetSum) && array.some((el2) => el2 + el === targetSum)
  );
}

function twoNumberSum2(array, targetSum) {
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

function twoNumberSum3(array, targetSum) {
  return array.reduce((acc, val) => {
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



function twoNumberSumLast(array, targetSum) {
  const targetSumTracker = new Map()
  for (const num of array) {

    const secondNum = targetSum - num;

    if (targetSumTracker.has(secondNum)) {
      return [num, secondNum]
    }

    targetSumTracker.set(num, true)


  }

  return []
}


function twoNumberSum(array, targetSum) {
  // Write your code here.
  const sortedArray = array.sort((a, b) => a - b);
  let left = 0;
  let right = array.length - 1

  while (left < right) {
    const currentSum = array[left] + array[right]
    console.log("array[right]", array[right])
    console.log("array[left]", array[left])
    console.log("left", left)
    console.log("right", right)
    console.log("currentSum", currentSum)
    if (currentSum === targetSum) {
      return [array[left], array[right]];
    } else if (currentSum < targetSum) {
      left++;
    } else if (currentSum > targetSum) {
      right--;
    }

  }

  return []
}


console.log("last", twoNumberSumLast(arr, tar));
