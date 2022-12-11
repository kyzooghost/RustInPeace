'''
Implement the following API, for a String SET data type:

This is a piece of cake compared to 5.2.5, it's just writing a wrapper class around either the TST or trie
'''

from iterative_trie import TrieST

trie = TrieST()

class StringSet:
    def __init__(self):
        self.trie = TrieST()

    def add(self, key: str):
        self.trie.put(key, 0)

    def delete(self, key: str):
        self.trie.delete(key, 0)

    def contains(self, key: str) -> bool:
        return self.trie.contains(key)

    def is_empty(self) -> bool:
        return self.trie.is_empty()

    def size(self) -> int:
        return self.trie.size