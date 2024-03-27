class Node:
    def __init__(self, data) -> None:
        self.data = data
        self.next = None


node1 = Node(24)
node2 = Node(11)
node3 = Node(53)
node4 = Node(234)

node1.next = node2
node2.next = node3
node3.next = node4


def find_lowest(head):
    current_node = head
    lowest = head

    while current_node:
        if current_node.data < lowest.data:
            lowest = current_node

        current_node = current_node.next

    return lowest


lowest = find_lowest(node1)
print((lowest.data))
