class Node {
    constructor(value){
        this.value = value;
    }
}

class Graph {
    constructor(){
        this.adjacencyList = {};
        // this.adjacencyList = new Map();
    }

    addNode = (node) => {
        if (!this.adjacencyList?.[node]) {
            this.adjacencyList[node] = []
        }
    }

    removeNode = (node) => {
        let nodesWithCurrentNode = this.adjacencyList[node];
        while (nodesWithCurrentNode.length) {
            let removedNode = nodesWithCurrentNode.pop();
            this.removeEdge(node, removedNode);
        }

        delete this.adjacencyList[node];
    }

    addEdge = (node1, node2, ) =>{
        // this.adjacencyList[node1].push(node2);
        // this.adjacencyList[node2].push(node1);
        if (!this.adjacencyList[node1].includes(node2)) {
            this.adjacencyList[node1].push(node2);
        }
        if (!this.adjacencyList[node2].includes(node1)) {
            this.adjacencyList[node2].push(node1);
        }
    }

    removeEdge = (node1, node2) => {
        this.adjacencyList[node1] = this.adjacencyList[node1].filter(n => n !== node2);
        this.adjacencyList[node2] = this.adjacencyList[node2].filter(n => n !== node1);
    }

    BFS = (start) => {
        let data = [];
        let queue = [start];
        let visited = {};
        visited[start] = true;
        let currentNode;


        while (queue.length) {
            currentNode = queue.shift();
            data.push(currentNode);

            this.adjacencyList[currentNode]?.forEach(dn =>{
                if (!visited[dn]) {
                    visited[dn] = true;
                    queue.push(dn);
                }
            });
        }

        return data;
    }

    DFS = (value) => {}
    
    DFSRecursive = (start) => {
        let data = [];
        let visited = {};
        visited[start] = true;

        const dfs = (node) =>{
            if (!node) return null;
            visited[node] = true;
            data.push(node);
            this.adjacencyList?.[node].forEach(neighbourNode=>{
                if(!visited[neighbourNode]){
                    // visited[neighbourNode] = true;
                    return dfs(neighbourNode);
                }
            });
        }

        dfs(start);
        return data;
    }

    DFSIterative = (start) => {
        let visited = {};
        let data = []
        const stack = [start];
        visited[start] = true;

        while (stack.length) {
            let currentNode = stack.pop();
            data.push(currentNode);

            this.adjacencyList?.[currentNode].forEach(neighbourNode =>{
                if (!visited[neighbourNode]) {
                    visited[neighbourNode] = true;
                    stack.push(neighbourNode)
                }
            });
        }

        return data;

    }
}



const graph = new Graph();
graph.addNode("HEL");
graph.addNode("TOR");
graph.addNode("EDM");
graph.addNode("CAL");
graph.addNode("PIT");
graph.addNode("NYC");

graph.addEdge("HEL", "TOR");
graph.addEdge("HEL", "EDM");
graph.addEdge("CAL", "EDM");
graph.addEdge("PIT", "EDM");
graph.addEdge("PIT", "CAL");
graph.addEdge("CAL", "TOR");
graph.addEdge("EDM", "NYC");

/* 

graph.addEdge("HEL", "TOR");
graph.addEdge("HEL", "EDM");
graph.addEdge("TOR", "CAL");
graph.addEdge("TOR", "EDM");
graph.addEdge("CAL", "PIT");
graph.addEdge("CAL", "EDM");
graph.addEdge("PIT", "EDM");
graph.addEdge("PIT", "NYC");
graph.addEdge("PIT", "CAL");
graph.addEdge("NYC", "HEL");
graph.addEdge("NYC", "PIT");
graph.addEdge("NYC", "CAL");
graph.addEdge("NYC", "EDM");
graph.addEdge("CAL", "TOR");
graph.addEdge("EDM", "NYC");
 */

// graph.removeEdge("HEL", "TOR")
// graph.addEdge("HEL", "TOR")
// graph.removeNode("HEL")

graph.addNode("A")
graph.addNode("B")
graph.addNode("C")
graph.addNode("D")
graph.addNode("E")
graph.addNode("F")


graph.addEdge("A", "B")
graph.addEdge("A", "C")
graph.addEdge("B","D")
graph.addEdge("C","E")
graph.addEdge("D","E")
graph.addEdge("D","F")
graph.addEdge("E","F")

console.log("adjacencyList", JSON.stringify(graph.adjacencyList, 2, 2));
console.log("graph.BFS(HEL))", graph.BFS("HEL"));
console.log("graph.DFSRecursive(HEL)", graph.DFSRecursive("HEL"));
console.log("graph.DFSIterative(HEL)", graph.DFSIterative("HEL"));

// let xx = new Map();
// xx.set("nama", []);
// xx.set("pop", "xwe");
// let p = xx.get("nama");
// p.push(54);
// p.push("3443");
// // xx.set("nama", xx.get("nama"));
// // xx.get("nama").push(5)
// // xx.get("nama").push("Wewe")
// console.log(xx)