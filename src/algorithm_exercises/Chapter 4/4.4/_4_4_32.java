/*
 * p689
 * Parent-checking heuristic. Modify Bellman-Ford to visit a vertex v only if its SPT parent edgeTo[v] is not currently on the queue. This heuristic has been reported by Cherkassky, Goldberg, and Radzik to be useful in practice. Prove that it correctly computes shortest paths and that the worst-case running time is proportional to EV.
 * 
 * If edgeTo[v] = w->v, then w has previously been relaxed, and w->v is the shortest path from source s->v found so far.
 * 
 * Then if we are now considering to relax v, and w is on the queue, w must have been revisited and put on the queue AFTER the initial relax that put v on the queue.
 * 
 * Hence distTo[w] must have decreased, after v was placed on the queue
 * 
 * If v was enqueued as a result of relaxing w, then it will be enqueued again on the second relaxation of w. Furthermore, distTo[v] reduce further on the second relaxation of w.
 * 
 * O(EV) time is trivial, it is a modified Bellman-Ford, and if we're skipping steps compared to the classic Bellman-Ford, then its worst case cannot be worse than Bellman-Ford
 * 
 */

public class _4_4_32 {
    
}
