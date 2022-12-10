'''
In-place key-indexed counting. Develop a version of key-indexed counting that uses only a constant amount of extra space. Prove that your version is stable or provide a counterexample.

Hmm if the restriction is using a constant amount of extra space, surely using the count and aux char arrays for each key-indexed count iteration counts?

Or is the question actually saying, don't use an aux array. And aux isn't a constant amount of extra space because it occupies the same amount of space as your original string array, hence its size is linear with the amount of words you want to sort. Whereas the count[] array is constant size given a constant radix or alphabetSize parameter.
'''

# Below written in Java

int alphabetSize = 256
int[] count = new int[alphabetSize + 1];
# startIndex[i] = Initial array position for alphabet character 'i', for any 'i' present in this digit iteration.
int[] startIndex = new int[alphabetSize + 1];

# Compute frequency counts
for (int i = 0; i < array.length; i++) {
    int digitIndex = array[i].value.charAt(digit);
    count[digitIndex + 1]++;
    startIndex[digitIndex + 1]++;

# Transform counts to indices
for (int r = 0; r < alphabetSize; r++) {
    count[r + 1] += count[r];
    startIndex[r + 1] += startIndex[r];

# Distribute
for (int i = 0; i < array.length; i++) {
    # Ahh I get it, so for the array[i] position, you keep putting that word into the right position (as determined by indices in count[]) and swap it for the word in that position. You repeat this until the array[i] position is occupied by the correct word - Determined by i is within [startIndex[digitIndex], count[digitIndex])
    while (true) {
        # Get index of character
        int digitIndex = array[i].value.charAt(digit);

        # If i is within [startIndex[digitIndex], count[digitIndex])
        # No need to swap anymore?
        if (startIndex[digitIndex] <= i && i < count[digitIndex]) {
            break;
        }

        # Get correct index for item
        int newIndex = count[digitIndex]++;

        # Swap into the right place, so what was at 'newIndex' now may be displaced
        ArrayUtil.exchange(array, i, newIndex);
    }
}
