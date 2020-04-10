class NodeBox {
  constructor(
    private data: unknown,
    private next: NodeBox = null,
    private prev: NodeBox = null
  ) {}

  setNextNode = (nextNode: NodeBox) => {
    this.next = nextNode;
  };

  getData = () => console.log(this.data);

  static countNodes = (head: NodeBox): number => {
    let nodeCount = 1;
    let currentNode = head;

    while (currentNode.next) {
      nodeCount++;
      currentNode = currentNode?.next;
    }
    return nodeCount;
  };
}

const node1 = new NodeBox(2);
const node2 = new NodeBox(4);
const node3 = new NodeBox(5);
const node4 = new NodeBox(6);
const node5 = new NodeBox(7);
const node6 = new NodeBox(7);
const node7 = new NodeBox(7);
const node8 = new NodeBox(7);
const node9 = new NodeBox(7);

node1.setNextNode(node2);
node2.setNextNode(node3);
node3.setNextNode(node4);
node4.setNextNode(node5);
node5.setNextNode(node6);
node6.setNextNode(node7);
node7.setNextNode(node8);

console.log(NodeBox.countNodes(node1));
