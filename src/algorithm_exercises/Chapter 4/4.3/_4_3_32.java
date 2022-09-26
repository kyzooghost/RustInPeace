/*
 * Specified set.
 * 
 * Given a connected edge-weighted graph G and a specified set of edges S (having no cycles), describe a way to find a minimum-weight spanning tree of G that contains all the edges in S.
 * 
 * Assuming that graph G has non-distinct weight edges, otherwise there is only one MST.
 * Also assuming that the specified set will create an eligible MST.
 * 
 * Insert all edges in set S into a minimum priority queue - setMinPQ.
 * Modify Kruskal's algorithm.
 * Whenever Kruskal's algorithm is considering an edge with the same weight as the top of setMinPQ.
 * 1. Find all equivalent weight edges
 * 2. Modify the order that Kruskal's algorithm considers the collection of equivalent weight edges - contents of setMinPQ first, then the remainder.
 * 
 */

public class _4_3_32 {
    
}
