class Node {
    constructor(val){
        this.val = val;
        this.next = null
    }


}

let first = new Node("Hi");
first.next = new Node("there");
first.next.next = new Node("how");
first.next.next.next = new Node("are");
first.next.next.next.next = new Node("you");



// console.log(JSON.stringify(first))


class SinglyLinkedList {
    // #value;
    // #next;
    // #head;
    // #tail;
    // #length;
    constructor(){
        this.head = null;
        this.tail = null;
        this.length = 0;
    }

    push = (value) => {
        let newNode = new Node(value);
        if(!this.head){
            // [2]
            this.head = newNode;
            this.tail = this.head;
        }else {
            // console.log("head", this.head);
            // bcos head = same object as tail. so when tail.next changes, it also affects head. since they point to same object
            // and we change the object attribute before changing/reassigning the object itself.
            // after reassignment, tail would now point to a new object other than same it was
            // previously pointing at as the head. 
            // [2, 3, 6]
            this.tail.next = newNode; 
            this.tail = newNode;
            // console.log("headafter", this.head);
        }
        this.length++;
        return this;

    }

    traverse =()  =>{
        let current = this.head;
        while(current){
            console.log(current.val);
            console.log("this.head", this.head);
            console.log("current", current);
            current = current.next;
        }
    }

    pop = () => {
        if(!this.head) return undefined;
        let current = this.head;
        let secondToLastNewTail = current;


        while(current.next){
            // temp = temp.ext;
            secondToLastNewTail = current;
            current = current.next;
            // console.log("current", current.val);
            // console.log("secondToLastNewTail", secondToLastNewTail.val);
            // console.log("prev", secondToLastNewTail);
        }
        // console.log("tailbefore", this.tail);
        // console.log("tailnextbefore", this.tail.next);
        this.tail = secondToLastNewTail;
        this.tail.next = null
        // console.log("tailafter", this.tail);
        // console.log("tailnextafter", this.tail.next);
        // console.log("secondToLastNewTail", secondToLastNewTail.val);
        this.length--;
        if (this.length === 0) {
            this.head = null;
            this.tail = null;
        }
        return current;
    }

    shift = ()=> {
        // if (!this.length === 0) {
        if (!this.head) {
            return undefined;
        }
        let currentHead = this.head;
        this.head = currentHead.next;
        this.length--;
        if (this.length === 0) {
            this.head = null;
            this.tail = null;
        }
        // this.head = null;
        return currentHead
    }

    unsift = ( val ) =>{
        let newHead = new Node(val);
        // let newHead = newNode();
        newHead.next = this.head;
        this.head = newHead;
        this.length ++;
        return this.head;
    }

    

}


let list = new SinglyLinkedList()
console.log(list.head)
list.push("HELLO");
console.log(list.head)
list.push("GOODBYE");
console.log(list.head)
list.push("!");
// list.traverse()
// console.log(list.head)
// list.pop()
// console.log(list.head)
// list.pop()
// console.log(list.head)
// list.pop()
// console.log(list.head)
// list.pop()
// console.log(list.head)

console.log("shift", list.shift())
console.log("B4LIST", list)
console.log("UNshift", list.unsift("LOWO"))

// console.log("shift", list.shift())
// console.log("shift", list.shift())
// console.log("shift", list.shift())
console.log("LIST", list)


// console.log(list.head);
// console.log(list.head.next);
// console.log(list.head.next.next);