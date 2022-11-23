/*
 * p688
 * All-pairs shortest path on a line. Given a weighted line graph (undirected con- nected graph, all vertices of degree 2, except two endpoints which have degree 1), devise an algorithm that preprocesses the graph in linear time and can return the distance of the shortest path between any two vertices in constant time.
 * 
 * You're allowed to 'preprocess' in linear time, so you can just compute the shortest path in the 'preprocessing step and save in memory.
 * 1. Add all edges and vertices
 * 2. Perform DFS from one of the two vertices with outdegree 1, set distTo[v] as dist from s to v.
 * 
 * 3. Shortest path between any two vertices 'v' and 'w' is |distTo[v] - distTo[w]|, which is an O(1) operation
 */

public class _4_4_31 {
    
}
