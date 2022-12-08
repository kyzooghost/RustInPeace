'''
p726
Develop an implementation of LSD string sort that works for variable-length strings.
'''

from typing import List

fruits = [
    'banana',
    'apple',
    'pineapple',
    'mango',
    'watermelon',
    'apple',
    'pear',
    'melon',
    'mandarin',
    'lime',
    'strawberry',
    'raspberry',
    'kiwifruit',
    'blueberry',
    'passionfruit',
]

radix = 256

# Find length of longest word
w = 0
for fruit in fruits:
    w = max(w, len(fruit))

# Iterate through each character, from the last backwards
# If character doesn't exist in a string, ignore it. We use an extra array which contains only the words with a character for the current index. As we iterate from character `w - 1` to character 0, this array grows bigger. This way we avoid 'out of index' errors
# This will be a O(wn) time complexity sort

# This is the array we mutate during each iteration of key-indexed counting
# We mutate this array instead of the original 'fruits' array to avoid accessing non-existent string indexes.
fruits_clone = []

for d in range(w - 1, -1, -1):
    # Add to fruits_clone if this is a word which is newly included in this iteration
    for fruit in fruits:
        if len(fruit) - 1 == d:
            fruits_clone.append(fruit)

    # Apply key-indexed counting on fruits_clone

    count = [0 for _ in range(257)]

    # Compute frequency counts
    for fruit in fruits_clone:
        character: str = fruit[d]
        characterAsAscii: int = ord(character)
        count[characterAsAscii + 1] += 1

    # Counts to indices
    for i in range(radix):
        count[i + 1] += count[i]

    # Distribute into aux[] array
    aux = ['' for _ in range(len(fruits_clone))]

    for i in range(len(fruits_clone)):
        character: str = fruits_clone[i][d]
        characterAsAscii: int = ord(character)
        index = count[characterAsAscii]
        aux[index] = fruits_clone[i]
        count[characterAsAscii] += 1
        # print(fruits_clone[i])

    # Copy from aux[] back into fruits_clone[]
    for i in range(len(fruits_clone)):
        fruits_clone[i] = aux[i]

print(fruits_clone)
print(sorted(fruits))