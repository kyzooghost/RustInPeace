
from typing import List;
from enum import Enum

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

class TST:
    class Node:
        def __init__(self):
            self.character = None
            self.value = None
            self.left = None
            self.mid = None
            self.right = None

    def __init__(self):
        self.root = self.Node()
        self.root.character = ''
        self.size = 0

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

    def put(self, key: str, value):

        # Start search from root node
        node = self.root

        d = 0
        while d < len(key):
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

            # Find next node to visit - look at left, middle and right
            if current_node.mid is not None:
                collectNodeQueue.enqueue(current_node.mid)
                collectStringQueue.enqueue(current_string + current_node.mid.character)

            # Only visit left and right if we have left the starting node
            if current_string != prefix:
                if current_node.left is not None:
                    collectNodeQueue.enqueue(current_node.left)
                    collectStringQueue.enqueue(current_string[:-1] + current_node.left.character)
                if current_node.right is not None:
                    collectNodeQueue.enqueue(current_node.right)
                    collectStringQueue.enqueue(current_string[:-1] + current_node.right.character)

        
        return queue.queue

    def keysThatMatch(self, stringToMatch: str) -> List[str]:
        queue = Queue()

        # Edge case of empty string -> return empty list
        if stringToMatch == '':
            return queue.queue

        # Start from sole node connected to root
        collectNodeQueue = Queue()
        collectNodeQueue.enqueue(self.root.mid)
        collectStringQueue = Queue()
        collectStringQueue.enqueue("")
            
        while collectNodeQueue.is_empty() is False:
            current_node = collectNodeQueue.dequeue()
            previous_string = collectStringQueue.dequeue()

            if current_node is None or len(previous_string) >= len(stringToMatch):
                continue

            current_string = previous_string + current_node.character
            # If currently at wildcard character
            if stringToMatch[len(previous_string)] == '*':
                if len(current_string) == len(stringToMatch) and current_node.value is not None:
                    # We only collect the string if we have reached the terminal character
                    queue.enqueue(current_string)                    
                else:
                    # We only want to 'go down the middle' if we have not reached the terminal character
                    collectNodeQueue.enqueue(current_node.mid)
                    collectStringQueue.enqueue(current_string)

                collectNodeQueue.enqueue(current_node.left)
                collectStringQueue.enqueue(previous_string)
                collectNodeQueue.enqueue(current_node.right)
                collectStringQueue.enqueue(previous_string)
            # Else not at wildcard character
            else:
                # Have reached terminal character
                if len(current_string) == len(stringToMatch) and current_node.value is not None and current_string[-1] == stringToMatch[-1]:
                        queue.enqueue(current_string)
                else:
                    if stringToMatch[len(previous_string)] < current_string[-1]:
                        collectNodeQueue.enqueue(current_node.left)
                        collectStringQueue.enqueue(previous_string)
                    elif stringToMatch[len(previous_string)] > current_string[-1]:
                        collectNodeQueue.enqueue(current_node.right)
                        collectStringQueue.enqueue(previous_string)
                    else:
                        collectNodeQueue.enqueue(current_node.mid)
                        collectStringQueue.enqueue(current_string)

        return queue.queue
    
    def longestPrefixOf(self, string: str) -> str:
        if string == '':
            return ''
        
        d = 0
        length = 0
        node = self.root.mid

        while d < len(string):
            if node is None:
                break

            if node.value is not None and node.character == string[d]:
                length = d
            
            if ord(string[d]) < ord(node.character):
                node = node.left
            elif ord(string[d]) > ord(node.character):
                node = node.right
            else:
                node = node.mid
                d += 1

        return string[0:length + 1]

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

        # for i in range(nodeStack.size()):
        #     print(nodeStack.items[i].character)
        #     print(directionStack.items[i])
        #     print('---')

        # Search hit, set node.value to None
        if node.value is not None:
            node.value = None
            self.size -= 1

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

        return node