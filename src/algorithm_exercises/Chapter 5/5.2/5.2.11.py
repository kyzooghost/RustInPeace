'''
Eliminate one-way branching (internal and external) TrieST.

To eliminate one-way branching, we do not allow nodes with a single link
Need to account for edge case with empty string
To eliminate external one-way branching - if we encounter a non-existent node on the path, just dump a node with the entire string
To eliminate internal one-way branching - we to enable nodes that can fork into two paths, with the ends having a common prefix
'''

from typing import List;
from enum import Enum
from iterative_trie import Stack, Queue
import os.path

class TrieST:
    radix = 256

    class Node:
        def __init__(self, radix: int):
            self.nibble = None
            self.value = None
            self.next = [None] * radix

    def __init__(self):
        self.root = self.Node(self.radix)
        self.root.nibble = ""
        self.size = 0

    def isEmpty(self) -> bool:
        return self.size == 0

    def contains(self, key: str) -> bool:
        if self.get(key) is None:
            return False
        else:
            return True

    def get(self, key: str):
        node = self._get(key)

        if node is None:
            return None
        else:
            return node.value

    def _get(self, key: str) -> Node:
        node = self.root
        current_string = self.root.nibble

        while len(current_string) < len(key):
            print(current_string)
            next_nibble_start = key[len(current_string)]
            node = node.next[ord(next_nibble_start)]

            if node is None:
                break
            
            current_string = current_string + node.nibble
        print(current_string)
        if node is None or current_string != key:
            return None
        else:
            return node

    def put(self, key: str, value):        
        nodeStack = Stack()
        node = self.root
        current_string = node.nibble

        # Initial search
        while len(current_string) <= len(key) and current_string != key:
            # Check if need to create a fork
            if current_string != key[0:len(current_string)]:
                # Find the longest common prefix from start of current node nibble -> This is a new inner node
                pre_nibble_string = current_string[:len(current_string) - len(node.nibble)]
                longest_common_prefix = os.path.commonprefix([node.nibble, key[len(pre_nibble_string):]])
                suffix_for_existing_node = node.nibble[len(longest_common_prefix):]
                suffix_for_new_node = key[len(longest_common_prefix):]

                # Create the new inner node - this will fork into the current node and new node
                new_inner_node = self.Node(self.radix)
                new_inner_node.nibble = longest_common_prefix
                new_inner_node.next[ord(suffix_for_existing_node[0:1])] = node

                # Create new node to store value
                new_inner_node.next[ord(suffix_for_new_node[0:1])] = self.Node(self.radix)
                self.size += 1
                new_inner_node.next[ord(suffix_for_new_node[0:1])].value = value
                new_inner_node.next[ord(suffix_for_new_node[0:1])].nibble = suffix_for_new_node

                # Alter nibble of current node
                node.nibble = suffix_for_existing_node

                # Alter parent to link to new inner node, rather than node
                parent_node = nodeStack.pop()
                parent_node.next[ord(longest_common_prefix[0:1])] = new_inner_node

                # print('pre_nibble_string: ', pre_nibble_string)
                # print('longest_common_prefix: ', longest_common_prefix)
                # print('suffix_for_existing_node: ', suffix_for_existing_node)
                # print('suffix_for_new_node: ', suffix_for_new_node)

                return new_inner_node.next[ord(suffix_for_new_node[0:1])]

            next_nibble_start = key[len(current_string)]

            # Found a None node - fill this node with the remaining characters, and return
            if node.next[ord(next_nibble_start)] is None:
                node.next[ord(next_nibble_start)] = self.Node(self.radix)
                self.size += 1
                node = node.next[ord(next_nibble_start)]
                node.value = value
                node.nibble = key[len(current_string):]
                return node

            nodeStack.push(node)
            node = node.next[ord(next_nibble_start)]
            current_string = current_string + node.nibble

        # Did not encounter fork requirement or search miss => search hit so return node
        return node

    def delete(self, key: str):
        # First search for node, but also keep track of the nodes to get there
        nodeStack = Stack()
        node = self.root
        current_string = self.root.nibble

        while len(current_string) < len(key):
            nodeStack.push(node)
            next_nibble_start = key[len(current_string)]
            node = node.next[ord(next_nibble_start)]

            if node is None:
                break
            
            current_string = current_string + node.nibble

        # Search miss, return
        if node is None:
            return None

        # Search hit, set node.value to None
        node.value = None
        self.size -= 1

        childrenFound = []

        # Go upstream and eliminate empty nodes
        while nodeStack.is_empty() is False:
            if node.value is not None:
                break

            # Check all children
            childrenFound = []
            for i in range(self.radix):
                # If found non-null child, we can stop removing nodes.
                if node.next[i] is not None:
                    childrenFound.append(i)
            
            # Use foundChild flag to break out of outer loop
            if len(childrenFound) > 0:
                break

            # Did not find non-null child, remove this node and repeat process for previous node
            nibbleStart = node.nibble[0:1]
            current_string = current_string[0:len(current_string) - len(node.nibble)]
            node = nodeStack.pop()
            node.next[ord(nibbleStart)] = None

        # If we created an internal branch
        if len(childrenFound) == 1 and node.value is None:

            # Go downstream until we find either i.) a fork (len(childrenFound) >= 2), or ii.) node.value is not None
            while len(childrenFound) == 1 and node.value is None:
                # Push known internal branch node into stack
                nodeStack.push(node)
                # Go one node downstream, and check it for internal branching
                node = node.next[childrenFound[0]]
                childrenFound = []

                for i in range(self.radix):
                    if node.next[i] is not None:
                        childrenFound.append(i)

            # Now 'node' is the most downstream node we can find without internal branching
            # We can consolidate upstream nodes into this node
            prefix_to_add = ''
            upstream_node = None

            # Go upstream until we find a node with no internal branching
            while nodeStack.is_empty() is False:
                upstream_node = nodeStack.pop()

                # Test upstream node for internal branching
                childrenFound = []

                for i in range(self.radix):
                    if upstream_node.next[i] is not None:
                        childrenFound.append(i)

                # print(childrenFound, node.value)

                if len(childrenFound) == 1 and upstream_node.value is None:
                    prefix_to_add = upstream_node.nibble + prefix_to_add
                    continue
                else:
                    break

            # If we have reached an upstream node with no internal branching
            if upstream_node is not None:
                # Drop the internal branches
                upstream_node.next[ord(prefix_to_add[0:1])] = node
                # Alter the node nibble
                node.nibble = prefix_to_add + node.nibble

        return node

trie = TrieST()
# trie.put('', 0)
n = trie.put('shells', 1)
n2 = trie.put('shellfish', 2)
n2 = trie.put('shellfish', 2)
n3 = trie.put('shela', 3)
trie.put('shellfishes', 4)

# print(trie.size)
trie.delete('shela')
# trie.delete('shells')
trie.delete('shellfish')
# trie.delete('caba')
print(trie.size)
print(trie.get('shells'))
print(trie.get('shellfish'))
print(trie.get('shela'))
print(trie.get('shellfishes'))
print(trie.get('abacus'))