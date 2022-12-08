'''
Give a trace for 3-way string quicksort for the keys
no is th ti fo al go pe to co to th ai of th pa

Partition keys into 3 section
Do this recursively

no is th ti fo al go pe to co to th ai of th pa

Let's use the first array item as the pivot

First call
[no] as pivot
    is
    fo
    al
    go
    co
    ai
    of

    no

    pa
    th
    th
    to
    to
    pe
    ti
    th

Recursive call for
is
fo
al
go
co
ai
of

    pivot = [is]
    
    fo
    al
    go
    co
    ai

    is

    of

Recursive call for 
fo
al
go
co
ai

    al
    co
    ai

    fo

    go

Recursive call for 
al
co
ai

    ai
    al
    co

Return sorted left subarray
    ai
    al
    co
    fo
    go
    is
    of
    no

Recursive call for right subarray
pa
th
th
to
to
pe
ti
th

    pa

    th
    th
    to
    to
    pe
    ti
    th

Recursive call for 
th
th
to
to
pe
ti
th
    
    pe
    ti

    th
    th
    th

    to
    to

Return sorted array
    pa
    pe
    ti
    th
    th
    th
    to
    to

Sorted array is
    ai
    al
    co
    fo
    go
    is
    of
    no
    pa
    pe
    ti
    th
    th
    th
    to
    to
'''