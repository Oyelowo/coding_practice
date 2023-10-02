class Node {
    constructor(value){
        this.value = value;
        this.next = null;
    }
}

class SinglyLinkedList {
    constructor(){
        this.head = null;
        this.tail = null;
        this.size = 0;
    }

    // add = () => {}
    push = (value) => {
        let newNode = new Node(value);
        if (!this.head) {
            this.head = newNode;
            this.tail = newNode;   
        }else{
            // [2]  => {
            //          prev == head: null, head.next: null, tail: null, tail.next: null
            //       current == head: 2, head.next: null, tail: 2, tail.next: null
            //      // set head = val, head.next = null, tail = val, tail.next = null
            // [2, X] => {
            //          prev == head: 2, head.next: null, tail: 2, tail.next: null
            //       current == head: 2, head.next: X, tail: X, tail.next: null
            //      // head.next = val, tail = val, tail.next = null
            //  }
            // [2, X, Y] => {
            //           prev == head: 2, head.next: X, tail: X, tail.next: null
            //        current == head: 2, head.next: X, tail: Y, tail.next: null
            // }

            this.tail.next = newNode
            this.tail = newNode;
        }
        this.size++
    }
    insertAt = () => {}
    removeAt = () => {}
    shift = () => {}
    pop = () => {
        if (!this.head) return undefined;
        let current = this.head;
        let newTail = current;
        while(current.next){
            newTail = current;
            current = current.next;
        }
        this.tail = null;
        this.size--;
    }
    unshift = () => {}
    reverse = () => {}
    rotate = () => {}
}

let sll = new SinglyLinkedList();
sll.push(5);
sll.push(6);
sll.push(8);
sll.push(13);
sll.push(94);

sll.pop();

console.log("ALL", JSON.stringify(sll, 2, 2));