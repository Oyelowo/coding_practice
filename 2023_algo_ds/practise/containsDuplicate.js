/**
 * @param {number[]} nums
 * @return {boolean}
 */
function containsDuplicate(nums) {
    const lookup = new Set();
    for(const n of nums){
        if(lookup.has(n)){
            return true;
        }else{
            lookup.add(n);
        }
    }

    return false;
}
// function containsDuplicate(nums) {
//    return nums.length !== new Set([...nums]).size
// }
// function containsDuplicate(nums) {
//     return !!Object.values(nums.reduce((acc, val) => {
//         if(acc?.[val]){
//             acc[val] += 1;
//         }else {
//             acc[val] = 1;
//         }
//         return acc; 
//     }, {}))
//     .find(v=>v>1);
// }






// var containsDuplicate = function(nums) {
//     let lookup = new Map();
//     let containsDup = false;
//     let map = nums.forEach(n=> {
//         lookup.set(n, (lookup.get(n) ?? 0 ) + 1);
//         if(lookup.get(n) > 1){
//             containsDup = true;
//         }
//     });
//     return containsDup;
    
// };