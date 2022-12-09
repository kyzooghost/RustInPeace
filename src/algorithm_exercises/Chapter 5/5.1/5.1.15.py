'''
Sublinear sort. Develop a sort implementation for int values that makes two passes through the array to do an LSD sort on the leading 16 bits of the keys, then does an insertion sort.
'''

from typing import List;
from builtins import map
import random

class Alphabet:
    def __init__(self, string: str):
        self.radix = len(string)
        self.charToIndicesMap = {}
        self.indicesToCharMap = {}

        for index, character in enumerate(string):
            self.charToIndicesMap[character] = index
            self.indicesToCharMap[index] = character

    def toChar(self, index: int) -> str:
        return self.indicesToCharMap[index]

    def toIndex(self, char: str) -> int:
        return self.charToIndicesMap[char]

    def contains(self, char: str) -> bool:
        if char in self.charToIndicesMap:
            return True
        else:
            return False

    def R(self) -> int:
        return self.radix

    def lgR(self) -> float:
        return math.log(self.radix)

    def toIndices(self, string: str) -> List[int]:
        return list(map(lambda x: self.charToIndicesMap[x], string))

    def toChars(self, indices: List[int]) -> List[int]:
        return list(map(lambda x: self.indicesToCharMap[x], indices))

# words is 
# Assume `words` is number in binary string representation
def LSDsortForFirst16Chars(words: List[str], alphabet: Alphabet):
    # Here, LSD from character 15 to character 0
    for d in range(15, -1, -1):
        # Apply key-indexed counting on words_clone
        count = [0 for _ in range(alphabet.R() + 1)]

        # Compute frequency counts
        for word in words:
            character = '0' if d >= len(word) else word[d]
            count[alphabet.toIndex(character) + 1] += 1

        # Counts to indices
        for i in range(alphabet.R()):
            count[i + 1] += count[i]

        # Distribute into aux[] array
        aux = ['' for _ in range(len(words))]

        for i in range(len(words)):
            character = '0' if d >= len(words[i]) else words[i][d]
            index = count[alphabet.toIndex(character)]
            aux[index] = words[i]
            count[alphabet.toIndex(character)] += 1

        # Copy from aux[] back into words[]
        for i in range(len(words)):
            words[i] = aux[i]

def insertion_sort(arr: List[int]):
    # Iterate over the array starting at the second element
    for i in range(1, len(arr)):
        # Save the current element
        curr = arr[i]

        # Iterate backwards over the array until we reach the first element or
        # find an element that is smaller than the current element
        j = i - 1
        while j >= 0 and arr[j] > curr:
            # Shift the larger element to the right
            arr[j + 1] = arr[j]
            j -= 1

        # Insert the current element in its correct position
        arr[j + 1] = curr

    return arr

def sublinearSort(numbers: List[int]):
    # Convert numbers to binary string representation
    numbersAsBinary = list(map(lambda x: format(x, 'b'), numbers))

    # Left pad all numbers with '0' such that all the same length
    w = 0
    for binaryNumber in numbersAsBinary:
        w = max(w, len(binaryNumber))

    for i, _ in enumerate(numbersAsBinary):
        numbersAsBinary[i] = numbersAsBinary[i].rjust(32, '0')

    LSDsortForFirst16Chars(numbersAsBinary, Alphabet('01'))

    for i, binaryNumber in enumerate(numbersAsBinary):
        numbers[i] = int(numbersAsBinary[i], 2)

    insertion_sort(numbers)

numbers = []
for _ in range(10):
    numbers.append(random.randint(65536, 4294967296))

sublinearSort(numbers)
print(numbers)