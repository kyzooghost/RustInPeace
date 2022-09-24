// Reverse-delete algorithm. 

// Develop an implementation that computes the MST as follows: Start with a graph containing all of the edges. Then repeatedly go through the edges in decreasing order of weight. For each edge, check if deleting that edge will disconnect the graph; if not, delete it. Prove that this algorithm computes the MST. What is the order of growth of the number of edge-weight compares performed by your implementation?

// Start with graph containing all edges
// Go through each edge in decreasing order of weight
// Check that edge will disconnect graph, if not, delete it

// O (E^2) time complexity
// Compute MST - By algorithm definition, the resulting graph will be connected, and no connected graph can be created with lower edge weights

import java_utils.EdgeWeightedGraph;
import java_utils.Edge;
import java_utils.PriorityQueue;
import java_utils.Queue;
import java_utils.PriorityQueue.Orientation;
import java_utils.SeparateChainingHashTable;

public class _4_3_24 {
    public class ReverseMST {
        private Queue<Edge> mst;
        private double weight;

        public ReverseMST(EdgeWeightedGraph graph) {
            mst = new Queue<Edge>();
            weight = 0.0;

            // Add all graph edges to maximum PQ
            SeparateChainingHashTable<String, Double> remainingEdges = new SeparateChainingHashTable<String, Double>(); // Keep track of remaianing edges
            PriorityQueue<Edge> maxPQ = new PriorityQueue<Edge>(Orientation.MAX);
            for (Edge e: graph.edges()) {
                maxPQ.insert(e);
                remainingEdges.put(_edgeToString(e), e.weight());
            }

            // Iterate through each edge in decreasing order
            while (!maxPQ.isEmpty()) {
                Edge edge = maxPQ.deleteTop(); // O(E lg E)

                // Check if edge will disconnect graph - DFS to see if all vertices reached in new graph
                // If not disconnect, means not deleted and hence will be in MST
                // O(E^2) because DFS iterated E times
                if (deletedEdgeWillDisconnectGraph(edge, graph.vertices(), remainingEdges)) {
                    weight += edge.weight();
                    mst.enqueue(edge);
                }
            }
        }

        private boolean deletedEdgeWillDisconnectGraph(Edge edge, int vertices, SeparateChainingHashTable<String, Double> remainingEdges) {
            boolean[] visited = new boolean[(vertices)];
            remainingEdges.delete(_edgeToString(edge)); // Delete edge from remaining edges collection

            // Create graph
            EdgeWeightedGraph graph = new EdgeWeightedGraph(vertices);
            for (String edgeString : remainingEdges.keys()) {
                int[] edgeArray = _stringToEdge(edgeString);
                double weight = remainingEdges.get(edgeString);
                graph.addEdge(new Edge(edgeArray[0], edgeArray[1], weight));
            }

            // DFS from vertex 0 to find all connected vertices.
            dfs(graph, visited, 0);

            for (boolean isVisited : visited) {
                if (!isVisited) {
                    remainingEdges.put(_edgeToString(edge), edge.weight()); // Re-insert into remaining edges, if will disconnect
                    return true;
                }
            }

            return false;
        }

        private void dfs(EdgeWeightedGraph graph, boolean[] visited, int vertex) {
            visited[vertex] = true;
            for (Edge e: graph.adjacent(vertex)) {
                if (!visited[e.other(vertex)]) {dfs(graph, visited, e.other(vertex));}
            }
        }

        private String _edgeToString(Edge edge) {
            int vertex1 = edge.either();
            int vertex2 = edge.other(vertex1);
            if (vertex1 < vertex2) {return String.format("%d-%d", vertex1, vertex2);}
            else {return String.format("%d-%d", vertex2, vertex1);}
        }

        private int[] _stringToEdge(String string) {
            int[] edges = new int[2];
            int counter = 0;
            String[] stringArray = string.split("-");
            for (String s : stringArray) {
                edges[counter] = Integer.parseInt(s);
                counter ++;
            }
            return edges;
        }

        public Iterable<Edge> edges() {return mst;}
        public double weight() {return weight;}
    }

    public static void main(String[] args) {
        _4_3_24 exercise = new _4_3_24();

        // p624
        EdgeWeightedGraph graph = new EdgeWeightedGraph(8);
        graph.addEdge(new Edge(0, 7, 0.16));
        graph.addEdge(new Edge(2, 3, 0.17));
        graph.addEdge(new Edge(1, 7, 0.19));
        graph.addEdge(new Edge(0, 2, 0.26));
        graph.addEdge(new Edge(5, 7, 0.28));
        graph.addEdge(new Edge(1, 3, 0.29));
        graph.addEdge(new Edge(1, 5, 0.32));
        graph.addEdge(new Edge(2, 7, 0.34));
        graph.addEdge(new Edge(4, 5, 0.35));
        graph.addEdge(new Edge(1, 2, 0.36));
        graph.addEdge(new Edge(4, 7, 0.37));
        graph.addEdge(new Edge(0, 4, 0.38));
        graph.addEdge(new Edge(6, 2, 0.40));
        graph.addEdge(new Edge(3, 6, 0.52));
        graph.addEdge(new Edge(6, 0, 0.58));
        graph.addEdge(new Edge(6, 4, 0.93));

        ReverseMST mst = exercise.new ReverseMST(graph);

        for (Edge edge : mst.edges()) {
            System.out.println(edge.either() + " - " + edge.other(edge.either()) + " : " + edge.weight());
        }
    }
}
