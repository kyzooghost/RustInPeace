// Given an MST for an edge-weighted graph G, suppose that an edge in G that does not disconnect G is deleted. Describe how to find an MST of the new graph in time proportional to E.

// Proportional to E?

// If edge was not in MST, then nothing to do

// If edge was in MST, find the two cuts (?BFS from vertex1 - O(V + E)), then iterate through edges connecting the two cuts - O(E)

// iterate between other edges connecting the two vertices connected by the removed edge and add the one with minimal weight.

public class _4_3_14 {
    
}