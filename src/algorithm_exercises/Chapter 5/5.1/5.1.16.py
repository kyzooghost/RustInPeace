'''
Linked-list sort. Develop a sort implementation that takes a linked list of nodes with String key values as argument and rearranges the nodes so that they appear in sorted order (returning a link to the node with the smallest key). Use 3-way string quicksort.

Convert linked list to a list
Sort the list
Create a new linked list from the sorted list
This is your list
'''

class LinkedListNode:
    def __init__(self, val):
        self.val = val
        self.next = None

class LinkedList:
    def __init__(self):
        self.head = None
        self.tail = None

    def add(self, val):
        # Create a new node with the given value
        node = LinkedListNode(val)

        # If the linked list is empty, set the new node as the head and tail
        if not self.head:
            self.head = node
            self.tail = node
        # Otherwise, add the new node to the end of the linked list
        else:
            self.tail.next = node
            self.tail = node

    def remove(self, val):
        # If the linked list is empty, return
        if not self.head:
            return

        # If the value is at the head of the linked list, remove the head
        if self.head.val == val:
            self.head = self.head.next

        # Otherwise, iterate over the linked list and find the value
        else:
            current = self.head
            while current.next and current.next.val != val:
                current = current.next

            # If the value is found, remove it
            if current.next and current.next.val == val:
                current.next = current.next.next

    def find(self, val):
        # If the linked list is empty, return False
        if not self.head:
            return False

        # Iterate over the linked list and find the value
        current = self.head
        while current and current.val != val:
            current = current.next

        # Return True if the value is found, otherwise return False
        return current is not None

    def to_list(self):
        list = []
        node = self.head
        while node.next is not None:
            list.append(node.val)
            node = node.next

        return list

    def sort(self):

linkedList = LinkedList()

fruits = [
    'banana',
    'apple',
    'pineapple',
    'mango',
    'watermelon',
    'apple',
    'pear',
    'melon',
    'mandarin',
    'lime',
    'strawberry',
    'raspberry',
    'kiwifruit',
    'blueberry',
    'passionfruit',
]

for fruit in fruits:
    linkedList.add(fruit)

linkedList.sort()

# print(linkedList.sort())