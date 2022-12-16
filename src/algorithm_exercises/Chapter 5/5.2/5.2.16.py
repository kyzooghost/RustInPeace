'''
Document similarity. Write a TST client with a static method that takes an int value L and two file names as command-line arguments and computes the L-similarity of the two documents: the Euclidean distance between the frequency vectors defined by the number of occurrences of each trigram divided by the number of trigrams. Include a static method main() that takes an int value L as command-line argument and a list of file names from standard input and prints a matrix showing the L-similarity of all pairs of documents.

L-similarity
Euclidean distance between frequency vectors

What is a trigram? Sequence of three words
So parse each document, and enter each trigram into the a TST

You can modify the TST put operation to make it accommodate a frequency count
- Modify the function parameters so that only a key is needed, remove the value parameter
- Do a search for the key, check the node.value parameter. If None, make it 1. Else if it is a number, add 1 to it.
- Then TST.get() will return the frequency count for that trigram, and None if frequency = 0

Then use frequency counts to compute a frequency vector
Then calculate the Euclidean distance between the frequency vectors
d = sqrt( (x1 - y1)^2 + (x2 - y2)^2 + ... + (xn - yn)^2 )

And what does the variable L do in this situation?
Ah so L-gram? As in L = 3 => Look for trigram, etc. Got it
'''