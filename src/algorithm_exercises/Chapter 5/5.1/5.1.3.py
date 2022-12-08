'''
Give a trace for MSD string sort for the keys

no is th ti fo al go pe to co to th ai of th pa

Similar to LSD, just in reverse order and does a recursive sort for the next character, rather than a for-loop.

no 
is 
th
ti
fo
al
go
pe
to
co
to
th
ai
of
th
pa

First character
    Character count
        a = 2
        c = 1
        f = 1 
        g = 1
        i = 1
        n = 1
        o = 1
        p = 2
        t = 6

    Character to index
        a = 2
        c = 3
        f = 4 
        g = 5
        i = 6
        n = 7
        o = 8
        p = 10
        t = 16

    Compute array
        al
        ai
        co
        fo
        go
        is 
        no 
        of
        pe
        pa
        th
        ti
        to
        to
        th
        th

    Recursive call for [a...]
        Character count
            i = 1
            l = 1

        Count to index
            i = 1
            l = 2

        Array
            ai
            al

    Recursive calls for c, f, g, i, n and o immediately return because already sorted if only have 1 element

    Recursive call for p similar as to for a

    Recursive call for t
        Character count
            h = 3
            i = 1
            o = 2

        Count to index
            h = 3
            i = 4
            o = 6

        Array
            th
            th
            th
            ti
            to
            to
'''

