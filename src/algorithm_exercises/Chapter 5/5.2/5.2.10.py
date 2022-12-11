'''
p755
Size. Implement very eager size() (that keeps in each node the number of keys in its subtree) for TrieST and TST.

Very eager = Maintain number of keys in subtrie as a node instance variables
'''

from typing import List;
from enum import Enum
from iterative_trie import Stack, Queue

class TrieST:
    radix = 256

    class Node:
        def __init__(self, radix: int):
            self.size = 0
            self.value = None
            self.next = [None] * radix

    def __init__(self):
        self.root = self.Node(self.radix)

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

    def size(self) -> int:
        return self.root.size

    def put(self, key: str, value):
        nodeStack = Stack()
        node = self.root
        d = 0
        while d < len(key):
            nodeStack.push(node)
            # Encounter None node => Insert empty Node there
            if node.next[ord(key[d])] is None:
                node.next[ord(key[d])] = self.Node(self.radix)
            
            node = node.next[ord(key[d])]
            d += 1

        # We inserted a new value => Adjust all sizes in the stack
        if node.value is None:
            node.size = 1

            for n in nodeStack.items:
                n.size += 1

        node.value = value 
        return node

    def isEmpty(self) -> bool:
        return self.size == 0

    def contains(self, key: str) -> bool:
        if self.get(key) is None:
            return False
        else:
            return True

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

        while nodeStack.is_empty() is False:
            if node.value is not None:
                break

            # Check all children
            for i in range(self.radix):
                # Stop if found non-null child
                if node.next[i] is not None:
                    break

            # Did not find non-null child, remove this node and repeat process for previous node
            d -= 1
            node = nodeStack.pop()
            node.next[ord(key[d])] = None

        # Adjust size parameter by -1 up the tree
        node.size -= 1

        while nodeStack.is_empty() is False:
            node = nodeStack.pop()
            node.size -= 1

        return node

class TST:
    class Node:
        def __init__(self):
            self.size = 0
            self.character = None
            self.value = None
            self.left = None
            self.mid = None
            self.right = None

    def __init__(self):
        self.root = self.Node()
        self.root.character = ''

    def get(self, key: str):
        node = self._get(key)

        if node is None:
            return None
        else:
            return node.value

    def _get(self, key: str): 
        node = self.root
        d = 0
        while d < len(key):
            # Search miss
            if node is None:
                break

            # If at root node, force traversal down the middle
            if node.character == '':
                node = node.mid
                continue

            if ord(key[d]) < ord(node.character):
                node = node.left
            elif ord(key[d]) > ord(node.character):
                node = node.right
            elif d < len(key) - 1:
                node = node.mid
                d += 1
            # Search hit
            else:
                break

        if node is None:
            return None
        else:
            return node

    def size(self) -> int:
        return self.root.size

    def put(self, key: str, value):
        nodeStack = Stack()

        # Start search from root node
        node = self.root

        d = 0
        while d < len(key):
            nodeStack.push(node)

            # If at root node, force traversal down the middle
            if node.character == '':
                if node.mid is None:
                    node.mid = self.Node()
                node = node.mid
                continue

            if node.character is None:
                node.character = key[d]

            if ord(key[d]) < ord(node.character):
                if node.left is None:
                    node.left = self.Node()
                node = node.left
            elif ord(key[d]) > ord(node.character):
                if node.right is None:
                    node.right = self.Node()
                node = node.right     
            elif d < len(key) - 1:
                if node.mid is None:
                    node.mid = self.Node()
                node = node.mid
                d += 1 
            else:
                break

        if node.value is None:
            node.size = 1

            for n in nodeStack.items:
                n.size += 1
        
        node.value = value
        return node

    def isEmpty(self) -> bool:
        return self.size == 0

    def contains(self, key: str) -> bool:
        if self.get(key) is None:
            return False
        else:
            return True

    def delete(self, key: str):
        if key == '':
            if self.root.value is None:
                return None
            else:
                node.value = None
                self.size -= 1

        class Direction(Enum):
            LEFT = 1
            MID = 2
            RIGHT = 3

        # First search for node, but also keep track of the nodes to get there
        nodeStack = Stack()
        directionStack = Stack()

        node = self.root.mid
        d = 0

        while d <= len(key):
            if node is None:
                break

            # Search hit
            if d == len(key) - 1 and node.character == key[-1]:
                break

            nodeStack.push(node)

            if ord(key[d]) < ord(node.character):
                node = node.left
                directionStack.push(Direction.LEFT)
            elif ord(key[d]) > ord(node.character):
                node = node.right
                directionStack.push(Direction.RIGHT)
            else:
                node = node.mid
                directionStack.push(Direction.MID)
                d += 1

        # Search miss, return
        if node is None:
            return None

        # Search hit, set node.value to None
        if node.value is not None:
            node.value = None

        while nodeStack.is_empty() is False:
            if node.value is not None:
                break

            # Check all children, if non-null child then nothing else to do
            if node.left is not None or node.mid is not None or node.right is not None:
                break

            # Did not find non-null child, remove this node and repeat process for previous node
            node = nodeStack.pop()
            direction = directionStack.pop()

            if direction == Direction.LEFT:
                node.left = None
            elif direction == Direction.RIGHT:
                node.right = None
            else:
                node.mid = None

        # Adjust size parameter by -1 up the tree
        node.size -= 1

        while nodeStack.is_empty() is False:
            node = nodeStack.pop()
            node.size -= 1

        # We did not start the stack with the root, also need to adjust its size
        self.root.size -= 1

        return node

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
trie.put('db', 8)
trie.put('e', 9)

print(trie.size())
trie.delete('e')
trie.delete('db')
trie.delete('de')
print(trie.size())

tst = TST()
tst.put('', 0)
tst.put('a', 1)
tst.put('b', 2)
tst.put('ba', 3)
tst.put('ac', 4)
tst.put('abc', 5)
tst.put('cab', 6)
tst.put('bac', 7)
tst.put('db', 8)
tst.put('db', 8)
tst.put('e', 9)
tst.put('f', 10)
print(tst.size())
tst.delete('f')
tst.delete('f')
tst.delete('f')
tst.delete('e')
tst.delete('db')
print(tst.size())
