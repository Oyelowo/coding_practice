class Node {
    constructor(value){
        this.value = value;
        this.left = null;
        this.right = null;
    }
}


class BreadthFirstTree {
    constructor(){
        this.root = null;
    }

    insert = (value) => {
        let newNode = new Node(value);
        if (this.root === null) {
            this.root = newNode;
            return this;
        }
        let current = this.root;
        while (true) {
            if (value === current.value) return undefined;
            if (value < current.value) {
                if(current.left === null) {
                    current.left = newNode;
                    return this
                }
                current = current.left;
            } else {
                if (current.right === null) {
                    current.right = newNode;
                    return this;
                }
                current = current.right;
            }
        }
    }

    find = (value) => {
        if (this.root === null) {
            return false
        }

        let found = false;
        let current = this.root;
        while (current && !found) {
            if (value > current.value) {;
                current = current.right;
            } else if(value< current.value){
                current = current.left;
            }else{
                found = true;
            }
        }
        return found;
    }

    BFS = () => {
        let currentNode = this.root;
        let data = [];
        let queue = [];

        queue.push(currentNode);

        while (queue.length) {
            currentNode = queue.shift();
            data.push(currentNode.value);
            if (currentNode.left) queue.push(currentNode.left);
            if (currentNode.right) queue.push(currentNode.right);
        }
        return data;
    }
}


let bfs = new BreadthFirstTree();
bfs.insert(5);
bfs.insert(6);
bfs.insert(4);
bfs.insert(41);


console.log(JSON.stringify(bfs));
console.log(bfs.find(41))
console.log(bfs.BFS())
