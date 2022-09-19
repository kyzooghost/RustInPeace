// Edge connectivity. A bridge in a graph is an edge that, if removed, would separate a connected graph into two disjoint subgraphs. A graph that has no bridges is said to be edge connected. Develop a DFS-based data type for determing whether a given graph is edge connected.

// https://www.youtube.com/watch?v=Rhxs4k6DyMM

import java.util.ArrayList;
import java.util.List;
import java_utils.Graph;

public class _4_1_36 {

    private class Edge {
        int vertex1;
        int vertex2;

        Edge(int vertex1, int vertex2) {
            this.vertex1 = vertex1;
            this.vertex2 = vertex2;
        }
    }

    private static int count;

    private int[] low; // low[v] = lowest discovery time of vertex accessible to vertex v
    private int[] time; // time[v] = time at which DFS finds vertex v

    private List<Edge> findBridges(Graph graph) {
        low = new int[graph.vertices()];
        time = new int[graph.vertices()];

        List<Edge> bridges = new ArrayList<>();

        for (int vertex = 0; vertex < graph.vertices(); vertex++) {
            low[vertex] = -1;
            time[vertex] = -1;
        }

        for (int vertex = 0; vertex < graph.vertices(); vertex++) {
            if (time[vertex] == -1) {
                dfs(graph, vertex, vertex, bridges);
            }
        }
        return bridges;
    }

    private void dfs(Graph graph, int currentVertex, int sourceVertex, List<Edge> bridges) {
        time[currentVertex] = count; // Update time[] with discovery time
        low[currentVertex] = count;
        count++;

        for (int neighbor : graph.adjacent(currentVertex)) {
            // If undiscovered neighbour
            if (time[neighbor] == -1) {
                dfs(graph, neighbor, currentVertex, bridges);

                // Unwind from dfs, propagate found back-edges
                low[currentVertex] = Math.min(low[currentVertex], low[neighbor]);

                // Check if no back-edge (otherwise low[] would have indicated finding a backedge)
                if (low[neighbor] > time[currentVertex]) {
                    bridges.add(new Edge(currentVertex, neighbor));
                }

            // If neighbor already discovered, (and ignoring direct parent) => Must be a back edge
            } else if (neighbor != sourceVertex) {
                // Update low[] array with discovery time of non-parent neighbor - accessible back edge (up ancestor chain).
                low[currentVertex] = Math.min(low[currentVertex], time[neighbor]);

                // No need check bridge here, back edge cannot be a bridge
            }
        }
    }

    public static void main(String[] args) {
        _4_1_36 exercise = new _4_1_36();
        Graph graph = new Graph(6);
        graph.addEdge(0, 1);
        graph.addEdge(0, 2);
        graph.addEdge(1, 2);
        graph.addEdge(2, 3);
        graph.addEdge(3, 4);
        graph.addEdge(4, 5);
        graph.addEdge(3, 5);

        List<Edge> edges = exercise.findBridges(graph);
        System.out.println(edges.size());
    }
}
