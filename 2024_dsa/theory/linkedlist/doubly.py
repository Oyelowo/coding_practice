class Node:
    def __init__(self, data) -> None:
        self.data = data
        self.next = None
        self.prev = None


node1 = Node(24)
node2 = Node(11)
node3 = Node(53)
node4 = Node(234)

node1.next = node2

node2.next = node3
node2.prev = node1

node3.next = node4
node3.prev = node2

node4.prev = node3


# Go forward
print("Forward")
current_node = node1
print(current_node.data)

while current_node.next is not None:
    current_node = current_node.next
    print(current_node.data)


# Go backward
print("===")
print("Backward")
last_node = node4
print(last_node.data)

while last_node.prev is not None:
    last_node = last_node.prev
    print(last_node.data)
