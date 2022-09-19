// Random interval graphs. Consider a collection of V intervals on the real line (pairs of real numbers). Such a collection defines an interval graph with one vertex corresponding to each interval, with edges between vertices if the corresponding intervals intersect (have any points in common). Write a program that generates V random in- tervals in the unit interval, all of length d, then builds the corresponding interval graph. Hint: Use a BST.

// Each interval => vertex
// Edge connects vertex with intersecting intervals
// BST store starting point of each interval

// Get all keys in order using BST.keys()
// Iterate amongst BST.keys(), if two adjacent entries are intersecting (a + d >= b), then addEdge on graph

public class _4_1_45 {
    
}
