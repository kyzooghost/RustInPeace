'''
Develop an implementation of key-indexed counting that makes use of an array
of Queue objects.

Array of Queues?
A queue for each letter?
'''
from typing import List;

# Ty ChatGPT
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

# Example usage

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

# Apply key-indexed counting on the first letter

# This is a much nicer implementation for key-indexed counting

# Create a queue for each character
radix = 256
queue_list: List[Queue] = []
for i in range(radix):
    queue_list.append(Queue())

# Add to queue for character, if character found in a word
for word in words:
    character:str = word[0]
    characterAsAscii: int = ord(character)
    queue_list[characterAsAscii].enqueue(word)

aux = []

# Iterate through queue list, and dequeue into the aux[] list
for queue in queue_list:
    if queue.size() > 0:
        while not queue.is_empty():
            aux.append(queue.dequeue())

print(aux)