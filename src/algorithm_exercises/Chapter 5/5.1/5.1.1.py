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
radix = 256

def sort(stringArray: List[str]):
    aux: List[str] = [None] * len(stringArray)
    _sort(stringArray, aux, 0, len(stringArray) - 1, 0)

def _sort(stringArray: List[str], aux: List[str], low: int, high: int, characterIndex: int):
    if high == low:
        return

    # Compute frequency counts
    # count[i + 2] = Frequency count for character with ascii code i, at characterIndex in stringArray
    count: List[int] = [0] * (radix + 2)
    for i in range(low, high + 1):
        character: str = stringArray[i][characterIndex]
        characterAsAscii: int = ord(character)
        count[characterAsAscii + 2] += 1

    # Transform counts to indices
    # count[i + 2] = Cumulative frequency count for character with ascii code <= i, at characterIndex in stringArray
    for i in range(0, radix + 1):
        count[i+1] += count[i]

    # Distribute from stringArray into aux
    for i in range(low, high + 1):
        character: str = stringArray[i][characterIndex]
        characterAsAscii: int = ord(character)
        # auxArrayPosition is the cumulative frequency of the previous ASCII character, not the current
        # Because the cumulative frequency of the previous ASCII character 
        # = ] bound of where previous ASCII character will extend to 
        # = ( bound of where the current ASCII character will begin from
        auxArrayPosition: int = count[characterAsAscii + 1]
        # We increment the count by 1 - we have used the current index in aux and need to use the next one
        # And we aren't using the entire 'count' array here actually, only the entries corresponding to 'ASCII_code - 1' => Seems like it would be more time and space efficient to use a hashmap?
        count[characterAsAscii + 1] += 1
        aux[auxArrayPosition] = stringArray[i]

    # Copy back from aux into stringArray
    for i in range(low, high + 1):
        # Use 'i - low' index in aux, we only use [0..len(stringArray) - low] indexes of aux
        stringArray[i] = aux[i - low]

    # Recursive sort for each character value
    for i in range(0, radix):
        # Skip for characters that weren't found at characterIndex
        if count[i + 1] == 0 or count[i + 1] - count[i] == 0:
            continue

        # count[i + 1] - 1 = last aux[] position occupied by character with ASCII code i at characterIndex
        firstAuxPosition = count[i]
        lastAuxPosition = count[i + 1] - 1
        # print(chr(i), count[i], count[i + 1] - 1)
        _sort(stringArray, aux, low + firstAuxPosition, low + lastAuxPosition, characterIndex + 1)

fruitList = list(fruits)
print(fruitList)
sort(fruitList)
print(fruitList)
