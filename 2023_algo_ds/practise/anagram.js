/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
// time - O(N), space = O(N)
// var isAnagram = function(s, t) {
//     if(s.length !== t.length) return false;  // time - O(1)
//     let ss = s.split("").sort();  // time - O(N + Nlog N)
//     let tt = t.split("").sort();  // time - O(N + Nlog N)
//     return ss.join("") === tt.join("");  // time - O(N + N) 
// };

function isAnagram (s, t) {
    if(s.length !== t.length) return false; 
    const lookup = new Map();

    for(const first of s) { 
        lookup.set(first, (lookup.get(first) ?? 0) + 1);
    }

    for(const second of t) { 
        lookup.set(second, (lookup.get(second) ?? 0) - 1);
    }

    for(const [k, v] of lookup) {
        if(v !== 0){
            return false
        }
    }
    return true;
}

// function isAnagram (s, t) {
//     if(s.length !== t.length) return false;
//     let left = s.split("");
//     let right = t.split("");
//     for(const l of left){
//         let rFoundIndex = right.indexOf(l);
//         if (rFoundIndex === -1){
//             return false
//         } else {
//             right[rFoundIndex] = null;
//         }
//     }

//     return true;
// };
// function isAnagram (s, t) {
//     if(s.length !== t.length) return false;
//     let left = s.split("");
//     let right = t.split("");
//     for(const l of left){
//         let rFound = right.find((x, i)=>{
//             if (x===l){
//                 right[i] = null;
//                 return true;
//             }
//             return false;
//         });
//         if(!rFound) return false;
//     }

//     return true;
// };