/** 
 * Critical Edges
 * 
 * An MST edge whose deletion from the graph would cause the MST weight to increase is called a critical edge.
 * Show how to find all critical edges in a graph in time proportional to E log E.
 * Note: This question assumes that edge weights are not necessarily distinct (otherwise all edges in the MST are critical).
 * 
 * Kruskal's algorithm has time complexity of O(E log E), hence we can 'piggyback' off Kruskal's to achieve O(E log E) time complexity
 * Edges in the MST with unique weight are critical.
 * Edges in the MST with a non-unique weight are possibly non-critical.
 * If adding all non-unique edges with the same weight to the subgraph (MST with all edges of lower weight + edges with same weight) creates a cycle, then edges involved in the cycle are non-critical
 * By definition, edges not a cycle are bridges
 * 
 * So we will take this approach:
 * Modify Kruskal's algorithm, so that whenever we encounter a collection of similar weight edges, we analyse which edges are critical. To perform this analysis:
 * 1. We reduce the edge-weighted graph to a graph of connected components (using UnionFind data structure)
 * 2. We perform Tarjan's bridge-finding algorithm to find all bridges in reduced graph (subgraph)
 * Note: Tarjan's bridge-finding algorithm fails for graphs with parallel edges, so we must filter out parallel edges. By definition parallel edges create a cycle between two vertices, so they are non-critical.
 */


import java_utils.Queue;
import java_utils.UnionFind;
import java_utils.PriorityQueue;
import java_utils.SeparateChainingHashTable;
import java_utils.Edge;
import java_utils.EdgeWeightedGraph;

public class _4_3_26 {

    public Queue<Edge> findCriticalEdges(EdgeWeightedGraph graph_) {
        // Collection of critical edges that we will return.
        Queue<Edge> criticalEdges = new Queue<>();

        // Modified Kruskal's algorithm.
        Queue<Edge> mst = new Queue<>();
        PriorityQueue<Edge> pq = new PriorityQueue<>(PriorityQueue.Orientation.MIN);
        UnionFind uf = new UnionFind(graph_.vertices());
        for (Edge e : graph_.edges()) { pq.insert(e); }

        while (!pq.isEmpty() && mst.size() < graph_.vertices() - 1) {
            Edge pqEdge = pq.deleteTop();
            int pqVertex1 = pqEdge.either();
            int pqVertex2 = pqEdge.other(pqVertex1);
            if(uf.connected(pqVertex1, pqVertex2)) continue;

            // Add all equivalent weight edges to equivalentWeightEdges collection.
            Queue<Edge> equivalentWeightEdges = new Queue<Edge>();
            equivalentWeightEdges.enqueue(pqEdge);

            while (!pq.isEmpty() && pq.peek().weight() == pqEdge.weight()) {
                equivalentWeightEdges.enqueue(pq.deleteTop());
            }

            // If unique weight edge, perform regular Kruskal's algorithm
            if (equivalentWeightEdges.size() == 1) {
                uf.union(pqVertex1, pqVertex2);
                mst.enqueue(pqEdge);

                // Unique weight edge is a critical edge.
                criticalEdges.enqueue(pqEdge);
                continue;
            }

            // Reduce EdgeWeightedGraph graph_ argument to a graph of connected components.
            EdgeWeightedGraph subgraph = new EdgeWeightedGraph(graph_.vertices());
            SeparateChainingHashTable<Edge, Edge> subgraphEdgeToGraphEdgeMap = new SeparateChainingHashTable<Edge, Edge>();

            // Keep track of parallel edges added.
            SeparateChainingHashTable<String, Integer> subgraphEdgeCounter = new SeparateChainingHashTable<String, Integer>();

            for (Edge originalGraphEdge : equivalentWeightEdges) {
                int subgraphVertex1 = uf.find(originalGraphEdge.either());
                int subgraphVertex2 = uf.find(originalGraphEdge.other(originalGraphEdge.either()));
                Edge subgraphEdge = new Edge(subgraphVertex1, subgraphVertex2, originalGraphEdge.weight());
                subgraph.addEdge(subgraphEdge);
                subgraphEdgeToGraphEdgeMap.put(subgraphEdge, originalGraphEdge);
                int subgraphEdgeCount = subgraphEdgeCounter.get(_edgeToString(subgraphEdge)) == null ? 0 : subgraphEdgeCounter.get(_edgeToString(subgraphEdge));
                subgraphEdgeCounter.put(_edgeToString(subgraphEdge), subgraphEdgeCount + 1);
            }

            // Find subgraph bridges and add to criticalEdges
            Iterable<Edge> subgraphBridges = findBridges(subgraph);
            for (Edge subgraphBridge : subgraphBridges) {

                // Skip parallel edges found in Tarjan's algorithm
                if (subgraphEdgeCounter.get(_edgeToString(subgraphBridge)) > 1) {
                    continue;
                }

                Edge originalGraphEdge = subgraphEdgeToGraphEdgeMap.get(subgraphBridge);
                criticalEdges.enqueue(originalGraphEdge);
            }

            // Apply Kruskal's algorithm to equivalentWeightEdges in original order
            while (!equivalentWeightEdges.isEmpty()) {
                Edge e = equivalentWeightEdges.dequeue();
                int vertex1 = e.either();
                int vertex2 = e.other(vertex1);
                if(uf.connected(vertex1, vertex2)) continue;
                uf.union(vertex1, vertex2);
                // Critical edges will not create a cycle, so don't need to worry about missing here.
                mst.enqueue(e);
            }
        }

        return criticalEdges;
    }

    // Tarjan's bridge finding algorithm.
    // Note: Will count a parallel edge as a bridge, so will need to avoid adding these to criticalEdges.
    private Iterable<Edge> findBridges(EdgeWeightedGraph graph_) {
        int[] time = new int[graph_.vertices()];
        int[] low = new int[graph_.vertices()];
        Queue<Edge> bridges = new Queue<Edge>();

        for (int vertex = 0; vertex < graph_.vertices(); vertex++) {
            low[vertex] = -1;
            time[vertex] = -1;
        }

        for (int vertex = 0; vertex < graph_.vertices(); vertex++) {
            if (time[vertex] == -1) {
                dfs(graph_, vertex, vertex, bridges, low, time, 0);
            }
        }

        return bridges;
    }

    private void dfs(EdgeWeightedGraph graph, int currentVertex, int sourceVertex, Queue<Edge> bridges, int[] low, int[] time, int count) {
        // Update time[] with discovery time
        time[currentVertex] = count; 
        low[currentVertex] = count;
        // Increment time for next dfs.
        count++;

        for (Edge e : graph.adjacent(currentVertex)) {
            int neighbor = e.other(currentVertex);

            // If undiscovered neighbour
            if (time[neighbor] == -1) {
                dfs(graph, neighbor, currentVertex, bridges, low, time, count);

                // Unwind from dfs, propagate found back-edges
                low[currentVertex] = Math.min(low[currentVertex], low[neighbor]);

                // Check if no back-edge (otherwise low[] would have indicated finding a backedge)
                if (low[neighbor] > time[currentVertex]) {
                    bridges.enqueue(e);
                }

            // If neighbor already discovered, (and ignoring direct parent) => Must be a back edge
            } else if (neighbor != sourceVertex) {
                
                // Update low[] array with discovery time of non-parent neighbor - accessible back edge (up ancestor chain).
                low[currentVertex] = Math.min(low[currentVertex], time[neighbor]);

                // No need check bridge here, back edge forms a cycle so cannot be a bridge.
            }
        }
    }

    private String _edgeToString(Edge edge) {
        int vertex1 = edge.either();
        int vertex2 = edge.other(vertex1);
        if (vertex1 < vertex2) {return String.format("%d-%d", vertex1, vertex2);}
        else {return String.format("%d-%d", vertex2, vertex1);}
    }

    public static void main(String[] args) {
        _4_3_26 exercise = new _4_3_26();

        // Graph defined by https://miro.medium.com/max/518/0*ejqQT4ihViZj_dPT.png
        EdgeWeightedGraph edgeWeightedGraph1 = new EdgeWeightedGraph(5);
        edgeWeightedGraph1.addEdge(new Edge(0, 1, 1));
        edgeWeightedGraph1.addEdge(new Edge(1, 2, 1));
        edgeWeightedGraph1.addEdge(new Edge(2, 3, 2));
        edgeWeightedGraph1.addEdge(new Edge(3, 0, 2));
        edgeWeightedGraph1.addEdge(new Edge(4, 0, 3));
        edgeWeightedGraph1.addEdge(new Edge(4, 3, 3));
        edgeWeightedGraph1.addEdge(new Edge(4, 1, 6));
        Queue<Edge> criticalEdges1 = exercise.findCriticalEdges(edgeWeightedGraph1);

        System.out.println("CRITICAL EDGES 1: ");
        // Only the two edges with weight 1 should are critical edges.
        for (Edge edge : criticalEdges1) {
            System.out.println(edge.either() + " - " + edge.other(edge.either()) + " : " + edge.weight());
        }

        // https://miro.medium.com/max/494/0*QfnsqYPfAOUM4y3W.png
        EdgeWeightedGraph edgeWeightedGraph2 = new EdgeWeightedGraph(4);
        edgeWeightedGraph2.addEdge(new Edge(0, 1, 1));
        edgeWeightedGraph2.addEdge(new Edge(1, 2, 1));
        edgeWeightedGraph2.addEdge(new Edge(2, 3, 1));
        edgeWeightedGraph2.addEdge(new Edge(3, 0, 1));
        Queue<Edge> criticalEdges2 = exercise.findCriticalEdges(edgeWeightedGraph2);

        System.out.println("CRITICAL EDGES 2: ");
        // Should have no critical edges
        for (Edge edge : criticalEdges2) {
            System.out.println(edge.either() + " - " + edge.other(edge.either()) + " : " + edge.weight());
        }
    }
}
