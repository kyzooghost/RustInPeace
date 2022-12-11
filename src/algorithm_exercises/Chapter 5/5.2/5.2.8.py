'''
5.2.8

Ordered operations for tries. Implement the floor(), ceil(), rank(), and select() (from our standard ordered ST API from Chapter 3) for TrieST.

floor() - largest key <= key
ceil() - smallest key >= key
rank() - number of keys < key
select() - key of rank k

First question, how do we compare keys. Let's just say we sum up the ASCII code equivalent of each character in the string.
'''

from typing import List;
from math import trunc
from string_sort import ThreewayQuickSort, Alphabet

class Stack:
    def __init__(self):
        # Create an empty list to store the stack elements
        self.items = []

    def push(self, item):
        # Add the item to the top of the stack
        self.items.append(item)

    def pop(self):
        # Check if the stack is empty
        if not self.items:
            return None

        # Remove and return the top item from the stack
        return self.items.pop()

    def peek(self):
        # Check if the stack is empty
        if not self.items:
            return None

        # Return the top item from the stack without removing it
        return self.items[-1]

    def is_empty(self):
        # Return True if the stack is empty, False otherwise
        return not bool(self.items)

    def size(self):
        # Return the number of items in the stack
        return len(self.items)

class Queue:
    def __init__(self):
        # Initialize an empty list to store the elements of the queue
        self.queue = []

    def enqueue(self, element):
        # Add an element to the end of the queue
        self.queue.append(element)

    def dequeue(self):
        # Remove and return the first element of the queue
        return self.queue.pop(0)

    def is_empty(self):
        # Return True if the queue is empty, False otherwise
        return len(self.queue) == 0

    def size(self):
        # Return the number of elements in the queue
        return len(self.queue)

class TrieST:
    radix = 256

    class Node:
        def __init__(self, radix: int):
            self.value = None
            self.next = [None] * radix

    def __init__(self):
        self.root = self.Node(self.radix)
        self.size = 0

    def get(self, key: str):
        node = self._get(key)

        if node is None:
            return None
        else:
            return node.value

    def _get(self, key: str) -> Node:
        node = self.root

        d = 0
        while d < len(key):
            if node is None:
                break
            
            node = node.next[ord(key[d])]
            d += 1

        if node is None:
            return None
        else:
            return node

    def put(self, key: str, value):
        node = self.root

        d = 0
        while d < len(key):
            # Encounter None node => Insert empty Node there
            if node.next[ord(key[d])] is None:
                node.next[ord(key[d])] = self.Node(self.radix)
            
            node = node.next[ord(key[d])]
            d += 1

        if node.value is None:
            self.size += 1

        node.value = value 
        return node

    def isEmpty(self) -> bool:
        return self.size == 0

    def contains(self, key: str) -> bool:
        if self.get(key) is None:
            return False
        else:
            return True

    def keys(self) -> List[str]:
        return self.keysWithPrefix('')

    def keysWithPrefix(self, prefix: str) -> List[str]:
        queue = Queue()
        node = self._get(prefix)
        if node is None:
            return queue.queue

        # Not only have to keep track of stack, but also the string
        collectNodeQueue = Queue()
        collectNodeQueue.enqueue(node)
        collectStringQueue = Queue()
        collectStringQueue.enqueue(prefix)

        while collectNodeQueue.is_empty() is False:
            current_node = collectNodeQueue.dequeue()
            current_string = collectStringQueue.dequeue()

            # Add value to queue if not null
            if current_node.value is not None:
                queue.enqueue(current_string)

            # Find next node to visit
            for i in range(self.radix):
                # If non-None Node then need to put on queue to visit
                if current_node.next[i] is not None:
                    collectNodeQueue.enqueue(current_node.next[i])
                    collectStringQueue.enqueue(current_string + chr(i))
        
        ThreewayQuickSort(queue.queue, Alphabet('abcdefghijklmnopqrstuvwxyz'))
        return queue.queue

    def keysThatMatch(self, stringToMatch: str) -> List[str]:
        queue = Queue()

        collectNodeQueue = Queue()
        collectNodeQueue.enqueue(self.root)
        collectStringQueue = Queue()
        collectStringQueue.enqueue("")
            
        while collectNodeQueue.is_empty() is False:
            current_node = collectNodeQueue.dequeue()
            current_string = collectStringQueue.dequeue()

            if current_node is None:
                continue

            if len(current_string) == len(stringToMatch) and current_node.value is not None:
                queue.enqueue(current_string)

            if len(current_string) == len(stringToMatch):
                continue

            for i in range(self.radix):
                if stringToMatch[len(current_string)] == '*' or stringToMatch[len(current_string)] == chr(i):
                    collectNodeQueue.enqueue(current_node.next[i])
                    collectStringQueue.enqueue(current_string + chr(i))

        return queue.queue

    def longestPrefixOf(self, string: str) -> str:
        d = 0
        length = 0
        node = self.root

        while d < len(string):
            if node is None:
                break

            if node.value is not None:
                length = d
            
            node = node.next[ord(string[d])]
            d += 1

        return string[0:length]

    def delete(self, key: str):
        # First search for node, but also keep track of the nodes to get there
        nodeStack = Stack()
        node = self.root
        d = 0

        while d < len(key):
            if node is None:
                break

            nodeStack.push(node)
            node = node.next[ord(key[d])]
            d += 1

        # Search miss, return
        if node is None:
            return None

        # Search hit, set node.value to None
        node.value = None
        self.size -= 1

        hasChild = False

        while nodeStack.is_empty() is False:
            if node.value is not None:
                break

            # Check all children
            for i in range(self.radix):
                if node.next[i] is not None:
                    hasChild = True
            
            # Found non-null child, nothing else to do
            if hasChild is True:
                break

            # Did not find non-null child, remove this node and repeat process for previous node
            d -= 1
            node = nodeStack.pop()
            node.next[ord(key[d])] = None

        return node
    
    # Number of keys that are < key
    def rank(self, key: str) -> int:
        keys = self.keys()
        order = {}

        low = 0
        high = len(keys) - 1

        while low <= high:
            mid = (low + high) // 2

            if key < keys[mid]:
                high = mid - 1
            elif key > keys[mid]:
                low = mid + 1
            else:
                return mid

        return low

    def select(self, rank: int) -> str:
        return self.keys()[rank]

    def floor(self, key: str) -> str:
        rank = self.rank(key)
        if rank == self.size or self.select(rank) != key:
            return self.select(rank - 1)
        else:
            return self.select(rank)

    def ceiling(self, key: str) -> str:
        rank = self.rank(key)
        if rank == self.size:
            raise Error('Key has no ceiling within current trie')
        return self.select(rank)

trie = TrieST()
trie.put('', 0)
trie.put('a', 1)
trie.put('b', 2)
trie.put('ba', 3)
trie.put('ac', 4)
trie.put('abc', 5)
trie.put('cab', 6)
trie.put('bac', 7)
trie.put('db', 8)

print(trie.keys())
print(trie.rank('db'))
print(trie.select(5))
print(trie.rank(''))
print(trie.floor('bab'))
print(trie.ceiling('bab'))
