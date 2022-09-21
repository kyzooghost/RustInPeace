// Directed Eulerian cycle. An Eulerian cycle is a directed cycle that contains each edge exactly once. Write a graph client Euler that finds an Eulerian cycle or reports that no such tour exists. 

// Hint : Prove that a digraph G has a directed Eulerian cycle if and only if G is connected and each vertex has its indegree equal to its outdegree.

/* 

Graph with no edges => Directed Eulerian cycle

Use property that digraph G can only be directed Eulerian cycle if i.) G connected, and ii.) each vertex must have same indegree as outdegree

Perform DFS, with modification that we don't mark vertices as visited 
Push each vertex of DFS exploration to array
Test if array length size == edges + 1

*/

public class _4_2_20 {
    
}
