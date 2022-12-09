'''
Queue sort. Implement MSD string sorting using queues, as follows: Keep one queue for each bin. On a first pass through the items to be sorted, insert each item into the appropriate queue, according to its leading character value. Then, sort the sublists and stitch together all the queues to make a sorted whole. Note that this method does not involve keeping the count[] arrays within the recursive method.

Err, this is just queue sort for key-indexed counting, iterated over each character
'''

from typing import List;

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

radix = 256

def sort(stringArray: List[str]):
    _sort(stringArray, 0, len(stringArray) - 1, 0)

def _sort(stringArray: List[str], low: int, high: int, characterIndex: int):
    if high == low:
        return

    if characterIndex >= len(stringArray[low]):
        return

    queue_list: List[Queue] = []
    for i in range(radix):
        queue_list.append(Queue())

    for i in range(low, high + 1):
        character:str = stringArray[i][characterIndex]
        characterAsAscii: int = ord(character)
        queue_list[characterAsAscii].enqueue(stringArray[i])

    aux = []

    for queue in queue_list:
        if queue.size() > 0:
            while not queue.is_empty():
                aux.append(queue.dequeue())

    # Copy back to original array
    for i in range(low, high + 1):
        stringArray[i] = aux[i - low]

    # Recursive sort for each present character

    # Account for next character being not present
    counter = low
    for i in range(radix):
        if ord(stringArray[counter][characterIndex]) != i:
            continue

        new_low = counter

        while counter < high and ord(stringArray[counter + 1][characterIndex]) == i:
            counter += 1

        new_high = counter

        _sort(stringArray, new_low, new_high, characterIndex + 1)
        
        if counter == high:
            break
        else:
            counter += 1


# print(words)
sort(words)
print(words)