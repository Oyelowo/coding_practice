function binarySearch(list, elem) {
    let start = 0;
    let end = list.length - 1;
    let middle = Math.floor((end + start) / 2);

    // [1,2,3,4,5]
    while((list[middle] !== elem) && start <= end ) {
        if (elem > list[middle]) {
            start = middle + 1
        }else{
            end = middle - 1;
        }
        middle = Math.floor((end + start) / 2);
    }

    if (list[middle] === elem) {
        // return list[middle]
        return middle
    }

    return -1;
}



console.log(binarySearch([2,5,6,9,13,15,28,30], 103))
console.log(binarySearch([2,5,6,9,13,15,28,30], 2))
console.log(binarySearch([2,5,6,9,13,15,28,30], 5))
console.log(binarySearch([2,5,6,9,13,15,28,30], 6))
console.log(binarySearch([2,5,6,9,13,15,28,30], 9))
console.log(binarySearch([2,5,6,9,13,15,28,30], 13))
console.log(binarySearch([2,5,6,9,13,15,28,30], 15))
console.log(binarySearch([2,5,6,9,13,15,28,30], 15))
console.log(binarySearch([2,5,6,9,13,15,28,30], 28))
console.log(binarySearch([2,5,6,9,13,15,28,30], 30))
console.log(binarySearch([2,5,6,9,13,15,28,30], 34))