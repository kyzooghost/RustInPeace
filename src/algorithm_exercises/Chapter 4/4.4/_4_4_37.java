/*
 * p690
 * Critical edges. Develop an algorithm for finding an edge whose removal causes maximal increase in the shortest-paths length from one given vertex to another given vertex in a given edge-weighted digraph.
 * 
 * Hmm, a critical edge. The critical edge must be one of the edges in the SPT. How do you decide which one? You could use the algorithm for source-sink shortest path repetitively, for the graph modified to omit an SPT edge.
 * 
 * So that is O(E * E lg V) time complexity. Can we do better?
 * 
 * ...
 * 
 * Mmm looks like a rather involved algorithm for O(V lg V) time complexity: requires new data structure - segment tree, running Dijkstra's algorithm twice, concept of 'islands' and 'bypass length'.
 */


public class _4_4_37 {

    // O(E lg V)
    public DirectedEdge getCriticalEdge(EdgeWeightedDigraph edgeWeightedDigraph, int source, int target) {
        // 1- Compute reverse digraph
        // It will be used to compute the shortest paths using the target vertex as a source.
        EdgeWeightedDigraph reverseDigraph = new EdgeWeightedDigraph(edgeWeightedDigraph.vertices());
        for (int vertex = 0; vertex < edgeWeightedDigraph.vertices(); vertex++) {
            for (DirectedEdge edge : edgeWeightedDigraph.adjacent(vertex)) {
                reverseDigraph.addEdge(new DirectedEdge(edge.to(), edge.from(), edge.weight()));
            }
        }

        // 2- Get shortest paths from source
        DijkstraSP dijkstraSP = new DijkstraSP(edgeWeightedDigraph, source);

        if (!dijkstraSP.hasPathTo(target)) {
            return null;
        }

        // 3- Get shortest paths from the target
        DijkstraSP dijkstraSPFromTarget = new DijkstraSP(reverseDigraph, target);

        // 4- Get the islands in the graph.
        // The i-th island is the set of all vertices v, such that there exists a shortest path
        // from source to v using no more than i shortest-path vertices.
        int[] islands = getIslands(edgeWeightedDigraph, dijkstraSP, source, target);

        // Each edge in the shortest path is a bridge
        // Categorise vertices in the graph, by lowest number of bridges between source and vertex


        // 5- Compute bypass path lengths
        SegmentTree bypassPathLengths = computeBypassPathLengths(edgeWeightedDigraph, islands, dijkstraSP,
                dijkstraSPFromTarget, target);

        // 6- Return a critical edge, which is an edge that has the highest bypass length
        return getCriticalEdge(bypassPathLengths, dijkstraSP, target);
    }

    private int[] getIslands(EdgeWeightedDigraph edgeWeightedDigraph, DijkstraSP dijkstraSP, int source, int target) {
        int[] islands = new int[edgeWeightedDigraph.vertices()];

        for (int vertex = 0; vertex < edgeWeightedDigraph.vertices(); vertex++) {
            islands[vertex] = -1;
        }

        int islandId = 0;

        // Traverse shortest path given by Dijstra's, and give islandId to each vertex encountered.
        for (DirectedEdge edge : dijkstraSP.pathTo(target)) {
            if (islands[edge.from()] == -1) {
                islands[edge.from()] = islandId++;
            }

            islands[edge.to()] = islandId++;
        }

        // Do a breadth-first walk to find the island number of vertices that are not on the shortest path
        // from source to target.
        // These vertices are on a path from source to target that is not a shortest path.
        boolean[] visited = new boolean[edgeWeightedDigraph.vertices()];
        Queue<Integer> queue = new Queue<>();
        queue.enqueue(source);
        visited[source] = true;

        // BFS - find all reachable vertices from current vertex
        // Expect islands[currentVertex] < islands[neighbor] if walking along shortest path
        // So 'if (islands[currentVertex] > islands[neighbor])' will trigger for every vertex not in shortest path, that is reachable from source. And if hasn't been found by BFS or Dijstra's yet, will be assigned islandId of currentVertex.
        // And because BFS, islands[currentVertex] will be assigned non-null value when placed on queue
        while (!queue.isEmpty()) {
            int currentVertex = queue.dequeue();

            for (DirectedEdge edge : edgeWeightedDigraph.adjacent(currentVertex)) {
                int neighbor = edge.to();

                if (!visited[neighbor]) {
                    visited[neighbor] = true;

                    if (islands[currentVertex] > islands[neighbor]) {
                        islands[neighbor] = islands[currentVertex];
                    }
                    queue.enqueue(edge.to());
                }
            }
        }

        return islands;
    }

    private SegmentTree computeBypassPathLengths(EdgeWeightedDigraph edgeWeightedDigraph, int[] islands,
                                                 DijkstraSP dijkstraSP, DijkstraSP dijkstraSPFromTarget, int target) {
        // bypassPathLengths[i] denotes the length of the shortest path that bypasses ei, where ei is the ith edge in the shortest path from source to target.
        double[] bypassPathLengths = new double[edgeWeightedDigraph.vertices()];
        // Ok so e_i is the ith bridge (edge in shortest path). e_i has islandId i.
        // Find bypassPassLength for every bridge

        for (int vertex = 0; vertex < edgeWeightedDigraph.vertices(); vertex++) {
            bypassPathLengths[vertex] = Double.POSITIVE_INFINITY;
        }

        HashSet<DirectedEdge> edgesInShortestPath = new HashSet<>();
        for (DirectedEdge edge : dijkstraSP.pathTo(target)) {
            edgesInShortestPath.add(edge);
        }

        SegmentTree segmentTree = new SegmentTree(bypassPathLengths);

        // Iterate through each edge not in shortest path
        for (DirectedEdge edge : edgeWeightedDigraph.edges()) {
            if (!edgesInShortestPath.contains(edge)) {
                // The edge is an alternative path between two islands - remember that the bridges are the fastest path between two islands by definition
                int island1 = islands[edge.from()];
                int island2 = islands[edge.to()];

                // Only if island1 < island2
                if (island1 < island2
                        && island1 != -1 && island2 != -1) {
                    
                    // For source s, edge v->w, target t
                    // Shortest path through edge is d(s->v) + d(v->w) + d(t->w)
                    double shortestPathLength = dijkstraSP.distTo(edge.from()) + edge.weight()
                            + dijkstraSPFromTarget.distTo(edge.to());

                    // What is our current shortest path between these two islands.
                    double currentShortestPathLength = segmentTree.rangeMinQuery(island1, island2 - 1);

                    if (shortestPathLength < currentShortestPathLength) {
                        // Hmm, range-update between island1 and island2 - 1?
                        // All values in bypassPathLengths[island1, island2] get updated with shortestPathLength
                        // Because bypassPathLengths[i] is shortest path bypassing bridge i
                        // And if edge goes from island1 to island2, it bypasses bridges island1, ..., island2 - 1
                        segmentTree.update(island1, island2 - 1, shortestPathLength);
                    }
                }
            }
        }

        return segmentTree;
    }

    private DirectedEdge getCriticalEdge(SegmentTree bypassPathLengths, DijkstraSP dijkstraSP, int target) {
        // key = edge in shortest path from source to target
        // value = id of edge in the path
        SeparateChainingHashTable<Integer, DirectedEdge> edgesInShortestPath = new SeparateChainingHashTable<>();
        int edgeIndex = 0;

        for (DirectedEdge edge : dijkstraSP.pathTo(target)) {
            edgesInShortestPath.put(edgeIndex++, edge);
        }

        int criticalEdgeId = 0;
        double highestBypassPathLength = Double.NEGATIVE_INFINITY;

        // bypassPathLengths.rangeSumQuery(edgeId, edgeId) is basically saying - get the value of bypassPathLengths[edgeId].
        for (int edgeId = 0; edgeId < edgeIndex; edgeId++) {
            if (bypassPathLengths.rangeSumQuery(edgeId, edgeId) > highestBypassPathLength) {
                highestBypassPathLength = bypassPathLengths.rangeSumQuery(edgeId, edgeId);
                criticalEdgeId = edgeId;
            }
        }

        return edgesInShortestPath.get(criticalEdgeId);
    }

}
