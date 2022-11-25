/*
 * p690
 * Sensitivity. Develop an SP client that performs a sensitivity analysis on the edge-weighted digraph’s edges with respect to a given pair of vertices s and t: Compute a V-by-V boolean matrix such that, for every v and w, the entry in row v and column w is true if v->w is an edge in the edge-weighted digraphs whose weight can be increased without the shortest-path length from v to w being increased and is false otherwise.
 * 
 * Run source-sink shortest path algorithm - O(E lg V).
 * Then if v->w is in the shortest path, false
 * Else true
 */


public class _4_4_33 {
}
