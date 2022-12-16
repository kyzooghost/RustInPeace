'''
p756
Substring matches. Given a list of (short) strings, your goal is to support queries where the user looks up a string s and your job is to report back all strings in the list that contain s. Design an API for this task and develop a TST client that implements your API. Hint : Insert the suffixes of each word (e.g., string, tring, ring, ing, ng, g) into the TST.

How do you use a TST for this?
If you insert string, tring, ring, ing, ng, g, how do you find 'tri'?
Well you look for 'tri', classic TST will return none. However if you call containsPrefix('tri') it will return true

So, is the answer just to insert suffix of each word that you insert, then call keysWithPrefix(x)?
Inserting each possible suffix for each word is very time inefficient - It makes put an O(N^2) operation instead of an O(N) operation, where N is the string length.

Yea so O(N^2) operation to put each possible suffix for the word, with actual word as the value
To find substring, run keysWithPrefix(substring)
'''