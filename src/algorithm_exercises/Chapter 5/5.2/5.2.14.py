'''
Unique substrings of length L. Write a TST client that reads in text from stan- dard input and calculates the number of unique substrings of length L that it contains. For example, if the input is cgcgggcgcg, then there are five unique substrings of length 3:cgc,cgg,gcg,ggc,andggg.Hint:Use the string method substring (i, i + L)to extract the ith substring, then insert it into a symbol table.

Ahh I got this
- You enter every L length substring into a TST - [0: 0 + L], [1: 1+ L] - and so on
- TST.size == number of unique substrings
'''