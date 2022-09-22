// p632

/*
Suppose that you use a priority-queue implementation that maintains a sorted list. What would be the order of growth of the worst-case running time for Prim’s algorithm and for Kruskal’s algorithm for graphs with V vertices and E edges? When would this method be appropriate, if ever? Defend your answer.
 */

/*
Does this mean to use a sorted list instead of a heap? So either insertion or deletion is O(N), rather than O(lg N) for both
So then it becomes O(E^2) to use in Prim's or Kruskal's algorithm?
Only appropriate if the edges are fed in already sorted order (so we are not paying the O(N lg N) cost for sorting, but someone else did)
 */
public class _4_3_19 {
    
}
