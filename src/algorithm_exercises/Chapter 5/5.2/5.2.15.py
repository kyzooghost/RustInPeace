'''
Unique substrings. Write a TST client that reads in text from standard input and calculates the number of distinct substrings of any length. This can be done very efficiently with a suffix tree — see Chapter 6

This is a more general case of the previous question.
You just use the solution of the previous question for every possible value of L - 1, 2, ..., len(string) - 1
'''