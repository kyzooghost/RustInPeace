'''
Alphabet. Develop an implementation of the Alphabet API that is given on page 698 and use it to develop LSD and MSD sorts for general alphabets.
'''

from typing import List;
from builtins import map
import math
import copy

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

alphabet = Alphabet('abcdefghijklmnopqrstuvwxyz')

words = [
    'is',
    'for',
    'all',
    'good',
    'come',
    'aid',
    'of',
    'now',
    'the',
    'to',
    'to',
    'people',
    'time',
    'the',
]

def MSDsort(words: List[str], alphabet: Alphabet):
    _MSDsort(words, 0, len(words) - 1, 0, alphabet)

def _MSDsort(stringArray: List[str], low: int, high: int, characterIndex: int, alphabet: Alphabet):
    if high == low:
        return

    if characterIndex >= len(stringArray[low]):
        return

    queue_list: List[Queue] = []
    for i in range(alphabet.R()):
        queue_list.append(Queue())

    for i in range(low, high + 1):
        character:str = stringArray[i][characterIndex]
        queue_list[alphabet.toIndex(character)].enqueue(stringArray[i])

    aux = []

    for queue in queue_list:
        if queue.size() > 0:
            while not queue.is_empty():
                aux.append(queue.dequeue())

    # Copy back to original array
    for i in range(low, high + 1):
        stringArray[i] = aux[i - low]

    # Recursive sort for each present character
    counter = low
    for i in range(alphabet.R()):
        if alphabet.toIndex(stringArray[counter][characterIndex]) != i:
            continue

        new_low = counter

        while counter < high and alphabet.toIndex(stringArray[counter + 1][characterIndex]) == i:
            counter += 1

        new_high = counter

        _MSDsort(stringArray, new_low, new_high, characterIndex + 1, alphabet)
        
        if counter == high:
            break
        else:
            counter += 1

def LSDsort(words: List[str], alphabet: Alphabet):
    # Find length of longest word
    w = 0
    for word in words:
        w = max(w, len(word))

    words_clone = []

    for d in range(w - 1, -1, -1):
        # Add to words_clone if this is a word which is newly included in this iteration
        for word in words:
            if len(word) - 1 == d:
                words_clone.append(word)

        # Apply key-indexed counting on words_clone
        count = [0 for _ in range(alphabet.R() + 1)]

        # Compute frequency counts
        for word in words_clone:
            character: str = word[d]
            count[alphabet.toIndex(character) + 1] += 1

        # Counts to indices
        for i in range(alphabet.R()):
            count[i + 1] += count[i]

        # Distribute into aux[] array
        aux = ['' for _ in range(len(words_clone))]

        for i in range(len(words_clone)):
            character: str = words_clone[i][d]
            index = count[alphabet.toIndex(character)]
            aux[index] = words_clone[i]
            count[alphabet.toIndex(character)] += 1

        # Copy from aux[] back into words_clone[]
        for i in range(len(words_clone)):
            words_clone[i] = aux[i]

    for i, _ in enumerate(words_clone):
        words[i] = words_clone[i]

    # Hmm so lists are passed by reference in Python
    # So if you mutate each individual element in the array (implicit dereferencing is happening here?) that mutation is persisted
    # But if you reassign the actual 'array' variable itself, you are only changing the local value of the 'array' reference. The original array is unaltered, and the original reference is maintaining outside the function scope.

def ThreewayQuickSort(words: List[str], alphabet: Alphabet):
    _ThreewayQuickSort(words, 0, len(words) - 1, 0, alphabet)

# Edge case - dealing with variable length string, treat non-existent character as special character that has index value of -1
def _ThreewayQuickSort(words: List[str], low: int, high: int, d: int, alphabet: Alphabet):
    if high <= low:
        return

    # Find length of longest word
    w = 0
    for word in words[low: high + 1]:
        w = max(w, len(word))
    if d >= w:
        return 

    left = low
    right = high

    partitionValue = -1 if d >= len(words[low]) else alphabet.toIndex(words[low][d])

    index = low + 1

    while index <= right:
        characterValue = -1 if d >= len(words[index]) else alphabet.toIndex(words[index][d])

        if characterValue < partitionValue:
            words[left], words[index] = words[index], words[left]
            index += 1
            left += 1
        elif characterValue > partitionValue:
            words[right], words[index] = words[index], words[right]
            right -= 1
        else:
            index += 1

    _ThreewayQuickSort(words, low, left - 1, d, alphabet)
    _ThreewayQuickSort(words, left, right, d + 1, alphabet)
    _ThreewayQuickSort(words, right + 1, high, d, alphabet)