'''
p755
Hybrid TST with R2-way branching at the root. Add code to TST to do multiway branching at the first two levels, as described in the text.

Array of R2 TST
This is much easier than removing one-way branching lol
Let's use small English characters only - a-z or ASCII code 97-122
'''

from iterative_trie import TST

class HybridTST:
    def __init__(self):
        self.size = 0
        self.tst_dict = {}
        for i in range (97, 123):
            for j in range (97, 123):
                self.tst_dict[chr(i) + chr(j)] = TST()


    def get(self, key: str):
        if len(key) < 2:
            raise Error('Cannot use key < 2 characters')
        return self.tst_dict[key[0:2]].get(key)

    def put(self, key: str, value):
        if len(key) < 2:
            raise Error('Cannot use key < 2 characters')
        tst_size = self.tst_dict[key[0:2]].size
        node = self.tst_dict[key[0:2]].put(key, value)
        if tst_size - self.tst_dict[key[0:2]].size == 1:
            self.size += 1
        return node
        
    def isEmpty(self) -> bool:
        return self.size == 0

    def contains(self, key: str) -> bool:
        if len(key) < 2:
            raise Error('Cannot use key < 2 characters')
        if self.get(key) is None:
            return False
        else:
            return True

    def delete(self, key: str):
        if len(key) < 2:
            raise Error('Cannot use key < 2 characters')
        tst_size = self.tst_dict[key[0:2]].size
        node = self.tst_dict[key[0:2]].delete(key)
        if tst_size - self.tst_dict[key[0:2]].size == 1:
            self.size += 1
        return node

hybridTST = HybridTST()
print(hybridTST.isEmpty())
hybridTST.put('ab', 0)
print(hybridTST.get('ab'))
hybridTST.delete('ab')
print(hybridTST.get('ab'))