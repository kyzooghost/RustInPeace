'''
Eliminate one-way branching (internal and external) for TST.

Use similar nibble strategy as for TrieST
'''

from typing import List;
from enum import Enum
from iterative_trie import Stack, Queue
import os.path

class TST:
    class Node:
        def __init__(self):
            self.nibble = None
            self.value = None
            self.left = None
            self.mid = None
            self.right = None

    class Direction(Enum):
        LEFT = 1
        MID = 2
        RIGHT = 3

    def __init__(self):
        self.root = self.Node()
        self.root.nibble = ''
        self.size = 0

    def get(self, key: str):
        node = self._get(key)

        if node is None:
            return None
        else:
            return node.value

    def _get(self, key: str): 
        node = self.root
        current_string = ''

        while len(current_string) < len(key):
            # Search miss
            if node is None:
                break

            # If at root node, force traversal down the middle
            if node.nibble == '':
                node = node.mid
                continue

            prospective_string = current_string + node.nibble

            # Search hit
            if prospective_string == key:
                break
            # Is substring -> Proceed down middle
            elif len(prospective_string) < len(key) and prospective_string == key[0: len(prospective_string)]:
                node = node.mid
                current_string = prospective_string
            # Compare nibble
            elif key[len(current_string):] < node.nibble:
                node = node.left
            elif key[len(current_string):] > node.nibble:
                node = node.right
            else:
                print('Gotten into forbidden land')

        if node is None:
            return None
        elif current_string + node.nibble != key:
            return None
        else:
            return node

    def put(self, key: str, value):
        node = self.root
        current_string = ''
        nodeStack = Stack()
        directionStack = Stack()

        while len(current_string) < len(key):
            # If at root node, force traversal down the middle
            if node.nibble == '':
                if node.mid is None:
                    node.mid = self.Node()
                nodeStack.push(node)
                directionStack.push(self.Direction.MID)
                node = node.mid
                continue

            # If encounter node.nibble == None node -> This is new end node
            if node.nibble is None:
                node.nibble = key[len(current_string):]
                node.value = value
                self.size += 1
                return node

            prospective_string = current_string + node.nibble

            # Search hit
            if prospective_string == key:
                node.value = value
                break
            # Is substring -> Proceed down middle
            elif len(prospective_string) < len(key) and prospective_string == key[0: len(prospective_string)]:
                if node.mid is None:
                    node.mid = self.Node()
                nodeStack.push(node)
                directionStack.push(self.Direction.MID)
                node = node.mid
                current_string = prospective_string
            # Check if fork - nibble and corresponding key part have common prefix, but is not substring
            elif os.path.commonprefix([node.nibble, key[len(current_string):]]) != '':
                longest_common_prefix = os.path.commonprefix([node.nibble, key[len(current_string):]])
                suffix_for_existing_node = node.nibble[len(longest_common_prefix):]
                suffix_for_new_node = key[len(current_string):][len(longest_common_prefix):]

                # Create new inner node
                new_inner_node = self.Node()
                new_inner_node.nibble = longest_common_prefix
                new_inner_node.mid = node

                # Place new inner node between previous visited node and node
                previous_node = nodeStack.pop()
                previous_direction = directionStack.pop()

                if previous_direction == self.Direction.LEFT:
                    previous_node.left = new_inner_node
                elif previous_direction == self.Direction.MID:
                    previous_node.mid = new_inner_node
                else:
                    previous_node.right = new_inner_node

                # Alter nibble of current node
                node.nibble = suffix_for_existing_node

                # Create new outer node to store value - this will be to the side of the current node
                self.size += 1
                if suffix_for_new_node > suffix_for_existing_node:
                    node.right = self.Node()
                    node.right.value = value
                    node.right.nibble = suffix_for_new_node
                    return node.right
                else:
                    node.left = self.Node()
                    node.left.value = value
                    node.left.nibble = suffix_for_new_node
                    return node.left

            # Compare nibble
            elif key[len(current_string):] < node.nibble:
                if node.left is None:
                    node.left = self.Node()
                nodeStack.push(node)
                directionStack.push(self.Direction.LEFT)
                node = node.left
            elif key[len(current_string):] > node.nibble:
                if node.right is None:
                    node.right = self.Node()
                nodeStack.push(node)
                directionStack.push(self.Direction.RIGHT)
                node = node.right
            else:
                print('Gotten into forbidden land')

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

        # First search for node, but also keep track of the nodes to get there
        nodeStack = Stack()
        directionStack = Stack()

        node = self.root.mid
        current_string = ''

        while len(current_string) < len(key):
            # Search miss
            if node is None:
                break

            # If at root node, force traversal down the middle
            if node.nibble == '':
                nodeStack.push(node)
                directionStack.push(self.Direction.MID)
                node = node.mid
                continue

            prospective_string = current_string + node.nibble

            # Search hit
            if prospective_string == key:
                break
            # Is substring -> Proceed down middle
            elif len(prospective_string) < len(key) and prospective_string == key[0: len(prospective_string)]:
                nodeStack.push(node)
                directionStack.push(self.Direction.MID)
                node = node.mid
                current_string = prospective_string
            # Compare nibble
            elif key[len(current_string):] < node.nibble:
                nodeStack.push(node)
                directionStack.push(self.Direction.LEFT)
                node = node.left
            elif key[len(current_string):] > node.nibble:
                nodeStack.push(node)
                directionStack.push(self.Direction.RIGHT)
                node = node.right
            else:
                print('Gotten into forbidden land')

        # Search miss, return
        if node is None:
            return None

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

            if direction == self.Direction.LEFT:
                node.left = None
            elif direction == self.Direction.RIGHT:
                node.right = None
            else:
                node.mid = None

        # Eager check if we have created vertical one-way branching
        if node.left is None and node.right is None and node.mid is not None and node.value is None:
            # If find vertical one-way branching, keep going down mid until find next node with no vertical one-way branching - merge one-way branch nodes into the downstream node with no one-way branching
            new_nibble = ''

            while node.left is None and node.right is None and node.mid is not None and node.value is None:
                new_nibble = new_nibble + node.nibble
                node = node.mid

            # Here node is downstream node with no one-way branching - consolidate nibble into this
            node.nibble = new_nibble + node.nibble

            # Remove all intermediate one-way branch nodes by changing link on node immediately upstream to the first vertical one-way branch we found
            upstream_node = nodeStack.peek()
            upstream_direction = directionStack.peek()
            
            if upstream_direction == self.Direction.LEFT:
                upstream_node.left = node
            elif upstream_direction == self.Direction.RIGHT:
                upstream_node.right = node
            else:
                upstream_node.mid = node

        # Check for left horizontal one way branching
        if node.left is not None and node.right is None and node.mid is None and node.value is None:
            while node.left is not None and node.right is None and node.mid is None and node.value is None:
                node = node.left

            upstream_node = nodeStack.peek()
            upstream_direction = directionStack.peek()
            
            if upstream_direction == self.Direction.LEFT:
                upstream_node.left = node
            elif upstream_direction == self.Direction.RIGHT:
                upstream_node.right = node
            else:
                upstream_node.mid = node   

        # Check for right horizontal one way branching
        if node.left is None and node.right is not None and node.mid is None and node.value is None:
            while node.left is None and node.right is not None and node.mid is None and node.value is None:
                node = node.right

            upstream_node = nodeStack.peek()
            upstream_direction = directionStack.peek()
            
            if upstream_direction == self.Direction.LEFT:
                upstream_node.left = node
            elif upstream_direction == self.Direction.RIGHT:
                upstream_node.right = node
            else:
                upstream_node.mid = node   

        # Horizontal one-way branching where node.left & node.right are not None, but node.mid and node.value are None - can choose to replace this with either left or right
        if node.left is not None and node.right is not None and node.mid is None and node.value is None:

            # We choose to replace this with node on the right that is not a horizontal one-way branch
            # So save node.left
            left_node = node.left

            while node.left is not None and node.right is not None and node.mid is None and node.value is None:
                node = node.right

            # Remove horizontal one-way branch nodes we have found
            node.left = left_node

            upstream_node = nodeStack.peek()
            upstream_direction = directionStack.peek()
            
            if upstream_direction == self.Direction.LEFT:
                upstream_node.left = node
            elif upstream_direction == self.Direction.RIGHT:
                upstream_node.right = node
            else:
                upstream_node.mid = node   

        return node

tst = TST()
n = tst.put('shells', 1)
n2 = tst.put('shellfish', 2)
n3 = tst.put('shela', 3)
n4 = tst.put('shellfishes', 4)
n5 = tst.put('shellz', 5)

a = tst.delete('shells')

print(tst.get('shells'))
print(tst.get('shellfish'))
print(tst.get('shela'))
print(tst.get('shellfishes'))
print(tst.get('shellz'))
print(tst.size)