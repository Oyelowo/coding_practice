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
node4.next = node1


# xxx
current_node = node1
head = node1
print(current_node.data)
while current_node.next != head:
    current_node = current_node.next
    print(current_node.data)
