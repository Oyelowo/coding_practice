class Node {
    constructor(val){
        this.val = val;
        this.next = null;
    }
}

const n1 = new Node(1);
const n2 = new Node(2);
const n3 = new Node(3);
const n4 = new Node(4);
const n5 = new Node(5);
const n6 = new Node(6);

n1.next = n2;
n2.next = n3;
n3.next = n4;
n4.next = n5;
n6.next = n6;

let n2_5 = new Node(2.5);
let old_n3 = n1.next.next;
// let after_n2_5 = n1.next.next.next;
let toSet = n1.next.next;
n1.next.next = n2_5;
n1.next.next.next = old_n3;



// const toSet = n1.next.next;
// const newNode = new Node(999);
// const prevHead = n1;
// n1.next.next = newNode;

console.log("n1", JSON.stringify(n1));
// console.log("prevHead", JSON.stringify(prevHead));
// console.log("toSet", JSON.stringify(toSet));
// console.log("n1.next.next", JSON.stringify(n1.next.next));




class TestRef {
    constructor(){
        this.student = {
            name : {}
        }
    }
}




// let std1 = new TestRef();
// std1.student.name.next = "lowo";

// let std2 = std1
// std2.student.name.next.xccc = "lowoxx";

// console.log("std1", std1);


// console.log("std2", std2);