class Node{
    constructor(val){
        this.val = val;
        this.next = null;
        this.prev = null;
    }
}


class DoublyLinkedList {
    constructor(){
        this.head = null;
        this.tail = null;
        this.length = 0;
    }
    push(val){
        var newNode = new Node(val);
        if(this.length === 0){
            this.head = newNode;
            this.tail = newNode;
        } else {
            this.tail.next = newNode;
            newNode.prev = this.tail;
            this.tail = newNode;
        }
        this.length++;
        return this;
    }
}
        


// DLL push - Exercise
// Implement the following on the DoublyLinkedList class

// push

// This function should accept a value add a node to the end of the DoublyLinkedList with the given value. It should return the DoublyLinkedList.

// Examples

var doublyLinkedList = new DoublyLinkedList;
doublyLinkedList.push(5); // doublyLinkedList
doublyLinkedList.length; // 1
doublyLinkedList.head.val; // 5
doublyLinkedList.tail.val; // 5
doublyLinkedList.head.prev // null
doublyLinkedList.push(10); doublyLinkedList 
doublyLinkedList.length; // 2
doublyLinkedList.head.val; // 5
doublyLinkedList.head.next.val; // 10
doublyLinkedList.tail.val; // 10
doublyLinkedList.head.next.prev.val // 10
doublyLinkedList.push(15); doublyLinkedList
doublyLinkedList.length; // 3
doublyLinkedList.head.val; // 5
doublyLinkedList.tail.val; // 15
doublyLinkedList.tail.prev.val; // 10
doublyLinkedList.head.next.next.val; // 15
 
doublyLinkedList.pop().val; // 15
doublyLinkedList.length; // 2
doublyLinkedList.pop().val; // 10
doublyLinkedList.length; // 1
doublyLinkedList.pop().val; // 5
doublyLinkedList.length; // 0
doublyLinkedList.pop(); // undefined
doublyLinkedList.length; // 0