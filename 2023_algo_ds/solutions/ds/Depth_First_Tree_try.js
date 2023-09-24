class Node {
    constructor(value){
        this.value = value;
        this.left = null;
        this.right = null;
    }
}


class BSTree {
    constructor(){
        this.root = null
    }

    insert = (value) => {
        let newNode = new Node(value);
        if (!this.root) {
            this.root = newNode;
            return this;
        }
        
        let current = this.root;
        while(true){
            if (value === current.value) return undefined;
            if (value < current.value) {
                if (current?.left === null) {
                    current.left = newNode;
                    return this;
                }
                current = current.left
            }else{
                if (current?.right === null) {
                    current.right = newNode;
                    return this;
                }
                current = current.right;
            }

        }
        // return 

    }

    find = (value)=>{
        if (!this.root) return false;
        // let node = new Node(value);
        let found = false;
        let current = this.root;
        while (current && !found) {
            if (value > current.value) {
                current = current.right;
            }else if(value < current.value) {
                current = current.left;
            } else{
                found = true;
            }
        }

        return found;
    }

    BFS = () => {
        if (!this.root) return undefined

        let currentNode = this.root;
        let data = [];
        let queue = [currentNode];

        while (queue.length) {
            if (!currentNode) return undefined;
            currentNode = queue.shift();
            data.push(currentNode.value);
            if (currentNode?.left) queue.push(currentNode.left);
            if (currentNode?.right) queue.push(currentNode.right);
        }

        return data
    }


    // BFSRecursive = () => {
    //     let data = [];

    //     bfs = (start) => {
    //         let current = start;
    //         data.push(current);
    //         if (current.left) bf
    //     }
    // }

    DFSIterativePreOrder = () => {
        let data = [];
        let currentNode = this.root;
        let stack = [currentNode];

        while(stack.length) {
            currentNode = stack.pop();
            data.push(currentNode.value);
            if (currentNode.left) stack.push(currentNode.left);
            if (currentNode.right) stack.push(currentNode.right);
        }

        return data
    }
    DFSIterativeInOrder = () => {
        let data = [];
        let currentNode = this.root;
        let stack = [currentNode];

        while(stack.length) {
            currentNode = stack.pop();
            if (currentNode.left) stack.push(currentNode.left);
            data.push(currentNode.value);
            if (currentNode.right) stack.push(currentNode.right);
        }

        return data
    }
    DFSIterativePostOrder = () => {
        let data = [];
        let currentNode = this.root;
        let stack = [currentNode];

        while(stack.length) {
            currentNode = stack.pop();
            if (currentNode.left) stack.push(currentNode.left);
            if (currentNode.right) stack.push(currentNode.right);
            data.push(currentNode.value);
        }

        return data
    }

    DFSRecursivePreOrder = () =>{
        let data = [];

        const dfs = (node) => {
            if (!node) return null;;
            data.push(node.value);
            if(node.left) dfs(node.left);
            if(node.right) dfs(node.right);
        }
        dfs(this.root)
        return data;
    }
    DFSRecursiveInOrder = () =>{
        let data = [];

        const dfs = (node) => {
            if (!node) return null;;
            if(node.left) dfs(node.left);
            data.push(node.value);
            if(node.right) dfs(node.right);
        }
        dfs(this.root)
        return data;
    }
    DFSRecursivePostOrder = () =>{
        let data = [];

        const dfs = (node) => {
            if (!node) return null;;
            if(node.left) dfs(node.left);
            if(node.right) dfs(node.right);
            data.push(node.value);
        }
        dfs(this.root)
        return data;
    }
}


let tree = new BSTree();
tree.insert(5);
tree.insert(4);
tree.insert(3);
tree.insert(7);
tree.insert(6);
tree.insert(70);
tree.insert(65);

//        5
//     4       7
//  3       6        70
//              65

// BFS => 5, 4, 7, 3, 6, 70, 65
// DFS => 5, 4, 3, 7, 6, 70, 65

console.log(JSON.stringify(tree, 2, 2));

console.log(tree.find(5));
console.log(tree.find(64));

console.log("DFSRecursivePreOrder =>", tree.DFSRecursivePreOrder());
console.log("DFSIterativePreOrder =>", tree.DFSIterativePreOrder());
console.log("======")

console.log("DFSRecursiveInOrder =>", tree.DFSRecursiveInOrder());
console.log("DFSIterativeInOrder =>", tree.DFSIterativeInOrder());
console.log("======")

console.log("DFSRecursivePostOrder =>", tree.DFSRecursivePostOrder());
console.log("DFSIterativePostOrder =>", tree.DFSIterativePostOrder());
console.log("======")

console.log("BFS =>", tree.BFS());
