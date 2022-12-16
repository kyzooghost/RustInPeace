'''
Random phone numbers. Write a TrieST client (with R = 10) that takes as command line argument an int value N and prints N random phone numbers of the form (xxx) xxx-xxxx. Use a symbol table to avoid choosing the same number more than once. Use the file AreaCodes.txt from the booksite to avoid printing out bogus area codes.

A client using TrieST class, to generate N random number of the form (xxx) xxx-xxxx
Use a symbol table to avoid choosing the same number more than once.

Pick a random area code
Generate 7 random digits
Check if already in TrieST
- If not put in TrieST
- If yes, go back to step 1

Repeat until TrieST.size == N
Use TrieST.keys() to get all N random numbers
'''