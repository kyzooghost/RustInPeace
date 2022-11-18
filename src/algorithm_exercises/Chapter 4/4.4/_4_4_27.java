/*
 * p688
 * Shortest paths in Euclidean graphs. Adapt our APIs to speed up Dijkstra’s algorithm in the case where it is known that vertices are points in the plane.
 * 
 * Do Dijkstra's algorithm - add edge to closest unincluded vertex, one at a time, to growing SPT
 * https://www.informit.com/articles/article.aspx?p=169575&seqNum=6
 * If vertices are points in the Euclidean plane, then:
 *  Triangle inequality: Given source s, sink t, vertex v. d(s, t) < d(s, v) + d(v, t). Where d(s, t) is the distance of a straight line between s and t.
 *  And d(s, t) is the lower bound for any path from s to t
 * 
 * Instead of initializing distTo[s] = 0, initialize as dist(s, t).
 * The relax condition also changes from 'distTo[w] > distTo[v] + v-w weight' 
 * to `distTo[w] > distTo[v] + d(v, w) + d(w, t) - d(v, t)` 
 * Because we are adding Eucl_dist(x, t) to each distTo[] term, we need to subtract Eucl_dist(x, t) to find distTo[w].
 * 
 * https://sedgewick.io/wp-content/themes/sedgewick/papers/1986Shortest.pdf
 * O(V) algorithm for SPT in Euclidean graph
 * distTo[x] = 'Distance of path from s to x' + 'Euclidean distance from x to t' = dist(s, x) + Eucl_dist(x, t)
 * So distTo[s] = 0 + Eucl_dist(s, t)
 * Each fringe vertex x is assigned value: min(dist(s, w) + Eucl_dist(w, x)) + Eucl_dist(x, t)
 *  w is a vertex already in the SPT
 *  Well dist(s, x) = dist(s, w) + Eucl_dist(w, x) anyway
 *  We are just adding Eucl_dist(x, t) at the end anyway
 *
 * Euclidean heuristic
 * SPT will grow in direction of t
 * Eucl_dist(x, t) gives a lower bound on dist(x, t)
 * 
 * So distTo[x] = Given the SPT so far, the lower bound on dist(s, t) through fringe vertex x.
 * Because the algorithm is centered on vertex t, distTo[t] will be the actual shortest path from s to t, however distTo[x] where x is any intermediate vertex in the shortest path, is the lower bound on the shortest path through x.
 * dist(s, x) = distTo[x] - Eucl_dist(x, t)
 * So instead of prioritizing by lowest 'dist(s, v)', we prioritize by lowest 'dist(s, v) + Eucl_dist(v, t)' or lower bound on path.
 * Which is smart, because it involves more precise prioritising - not just by the distance already taken, but by the distance already taken + lowest possible future distance.
 * 
 * You are adding edge to the fringe vertex with the lowest potential dist(s, t) associated with it, one at a time to the SPT.
 * 
 * Eucl_dist(x, t) is just one ways of providing a lower bound on dist(x, t). Case of A* algorithm for Euclidean graph.
 * 
 */