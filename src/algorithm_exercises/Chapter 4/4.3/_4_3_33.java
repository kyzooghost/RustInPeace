/*
 * Certification. p634.
 * 
 * Write an MST and EdgeWeightedGraph client check() that uses the following cut optimality conditions implied by Proposition J to verify that a proposed set of edges is in fact an MST: A set of edges is an MST if it is a spanning tree and every edge is a minimum-weight edge in the cut defined by removing that edge from the tree. What is the order of growth of the running time of your method?
 * 
 * Check if spanning tree - Create set of all vertices in edge set, check that set size == vertices size. O(V)
 * Check that every edge is a minimum-weight edge. For each edge in MST - O(V):
 *   Temporarily delete from MST as an EdgeWeightedGraphWithDelete
 *   Find all crossing edges 
 *      Add all EdgeWeightedGraphWithDelete edges to a UnionFind structure
 *      Iterate through each edge to check which are crossing edges (find(vertex1), find(vertex2)) => O(V * E) chokepoint
 *      Verify that out of all discovered crossing edge, outer loop edge has the minimum weight.
 */

public class _4_3_33 {
    
}
