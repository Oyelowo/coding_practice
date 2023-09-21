class MaxBinaryHeaps {
    constructor(){
        this.values = []
    }

    insert(val){
        this.values.push(val);
        this.bubbleUp();
    }

    bubbleUp=()=>{
        let idx = this.values.length - 1;
        let element = this.values[idx];
        while (idx > 0) {
            // console.log("dfd", lastIndex);
            // swap(lastIndex, parentIndex)
            let parentIndex = Math.floor((idx - 1) / 2);
            let parent = this.values[parentIndex]; 
            if (element <= parent) break;
            this.values[parentIndex] = element;
            this.values[idx] = parent;
            // [this.value[parentIndex], this.value[lastIndex]] = [this.value[lastIndex], this.value[parentIndex]];
            idx = parentIndex;
        }

    }
}

let bin = new MaxBinaryHeaps();
bin.insert(31);
bin.insert(41);
bin.insert(21);
bin.insert(61);
bin.insert(71);
bin.insert(81);
bin.insert(44);
bin.insert(60);

console.log("bin1", bin.values);

// bin.
// console.log("bin2", bin);



// class MaxBinaryHeap {
//     constructor(){
//         this.values = [];
//     }
//     insert(element){
//         this.values.push(element);
//         this.bubbleUp();
//     }
//     bubbleUp(){
//         let idx = this.values.length - 1;
//         const element = this.values[idx];
//         while(idx > 0){
//             let parentIdx = Math.floor((idx - 1)/2);
//             let parent = this.values[parentIdx];
//             if(element <= parent) break;
//             this.values[parentIdx] = element;
//             this.values[idx] = parent;
//             idx = parentIdx;
//         }
//     }
// }

// let heap = new MaxBinaryHeap();
// heap.insert(41);
// heap.insert(39);
// heap.insert(33);
// heap.insert(18);
// heap.insert(27);
// heap.insert(12);
// heap.insert(55);


