/*
 * Dense graphs. p634.
 * 
 * Develop an implementation of Prim’s algorithm that uses an eager approach (but not a priority queue) and computes the MST using V^2 edge-weight comparisons.
 * 
 * Outer loop - Iterate through each vertex, use marked[] array to avoid re-visiting the same vertex
 * Inner loop - Iterate through each adjacent edge to the outer loop vertex, update edgeTo[] and distTo[] arrays 
 * Inner loop - Iterate through each vertex, find the closest unvisited edge and mark as the next vertex to iterate in outer loop (This is an O(V) time efficiency for this inner-loop, when using a priority queue would be O(lg V) time efficiency)
 * 
 */


public class _4_3_29 {
    
}
