/*
 * p688
 * All-pairs shortest path in graphs with negative cycles. Articulate an API like the one implemented on page 656 for the all-pairs shortest-paths problem in graphs with no negative cycles. Develop an implementation that runs a version of Bellman-Ford to identify weights pi[v] such that for any edge v->w, the edge weight plus the difference between pi[v] and pi[w] is nonnegative. Then use these weights to reweight the graph, so that Dijkstra’s algorithm is effective for finding all shortest paths in the reweighted graph.
 * 
 * The textbook problem description was unclear to me. Reading the github solution, it appears that we're asking for 'all-pairs shortest path in a graph with negative weights' as opposed to 'multisource shortest paths' as 4.4.24 asked for.
 * 
 * 1. Take your graph, and add a dummy vertex that has an edge of weight 0 to every existing vertex.
 * 2. Run Bellman-Ford on this modified graph with a dummy vertex, with the dummy vertex as the source.
 * 3. Recreate the original graph, but with edge weights adjusted to 'original_edge_weight + pi[v] - pi[w]', where pi[v] is distTo[v] in the Bellman-Ford analysis in the above step. Apply Dijstra's algorithm.
 * 
 * I don't understand why original_edge_weight + pi[v] - pi[w] > 0 though
 * Well if v->w was a negative edge, the shortest distance pi[w] must go through v->w. And if the edges are relaxed, then pi[w] = original_edge_weight + pi[v].
 * pi[w] cannot > original_edge_weight + pi[v] or else this means Bellman-Ford was not applied properly. Thus original_edge_weight + pi[v] - pi[w] cannot < 0. Ok.
 * 
 * 4. Take the SPT generated in step 4 by Dijstra's algorithm, and 'unmodify each edge'. You now have an SPT in a graph with negative cycles.
 * 
 */

public class _4_4_30 {
    
}
