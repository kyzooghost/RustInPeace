# Here we replace the count[] array in the textbook MSD implementation with two hashmaps. count[] array is sparse af when dealing with standard English alphanumerical characters. Hashmaps (at least with linear probing or open-addressing) do involve a sparse array under the surface, but this is abstracted away from us when we use the library implementation :)
# Also open-addressing hashmaps have better performance in general vs closed addressing as i.) better memory locality when all the values are stored in a single array, ii.) Nil overhead with maintaining Linked List data structures

# p726

# Develop a sort implementation that counts the number of different key values, then uses a symbol table to apply key-indexed counting to sort the array. (This method is not for use when the number of different key values is large.)

from typing import List;

# Use a hashset to count the number of different key values
fruits = {
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
}

print(len(fruits), 'different fruits')

# Implement an MSD string sort
# Hmm, it is probably more space efficient to pass the same aux array and re-use it between calls, but I'd argue creating a new aux array with each call is more maintainable and developer friendly.

def sort(stringArray: List[str]):
    _sort(stringArray, 0, len(stringArray) - 1, 0)

def _sort(stringArray: List[str], low: int, high: int, characterIndex: int):
    if high == low:
        return

    # Compute frequency counts
    asciiToFrequency: dict[int, int] = dict({})
    for i in range(low, high + 1):
        character: str = stringArray[i][characterIndex]
        characterAsAscii: int = ord(character)
        if characterAsAscii in asciiToFrequency:
            asciiToFrequency[characterAsAscii] += 1
        else:
            asciiToFrequency[characterAsAscii] = 1

    # Compute cumulative frequency counts
    sortedPresentAscii = list(asciiToFrequency.keys())

    # Here we are introducing an O(N lg N) step, where N = # of different characters. I see why they said 'not for use when number of different key values is large'
    sortedPresentAscii.sort()
    startingAuxIndexForAscii: dict[int, int] = dict({})
    leftmostAuxIndexForAscii: dict[int, int] = dict({})

    runningCumulativeFrequency:int = 0
    for asciiCode in sortedPresentAscii:
        startingAuxIndexForAscii[asciiCode] = runningCumulativeFrequency
        leftmostAuxIndexForAscii[asciiCode] = runningCumulativeFrequency
        runningCumulativeFrequency += asciiToFrequency[asciiCode]

    # Shuffle stringArray into aux, using indices obtained from previous step
    aux = [None] * (high + 1 - low)

    for i in range(low, high + 1):
        character: str = stringArray[i][characterIndex]
        characterAsAscii: int = ord(character)
        auxIndex = leftmostAuxIndexForAscii[characterAsAscii]
        aux[auxIndex] = stringArray[i]
        leftmostAuxIndexForAscii[characterAsAscii] += 1

    # Copy back from aux into stringArray
    for i in range(low, high + 1):
        # Use 'i - low' index in aux, we only use [0..len(stringArray) - low] indexes of aux
        stringArray[i] = aux[i - low]

    # Recursive sort for each character value
    for asciiCode in sortedPresentAscii:
        firstAuxPosition = startingAuxIndexForAscii[asciiCode]
        lastAuxPosition = leftmostAuxIndexForAscii[asciiCode] - 1
        _sort(stringArray, low + firstAuxPosition, low + lastAuxPosition, characterIndex + 1)

fruitList = list(fruits)
# print(fruitList)
sort(fruitList)
print(fruitList)