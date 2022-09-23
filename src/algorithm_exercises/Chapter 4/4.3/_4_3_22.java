// p633
// Minimum spanning forest. 

// Develop versions of Prim’s and Kruskal’s algorithms that compute the minimum spanning forest of an edge-weighted graph that is not necessarily connected. Use the connected-components API of Section 4.1 and find MSTs in each component.

import java_utils.Queue;
import java_utils.Edge;
import java_utils.PriorityQueue;
import java_utils.EdgeWeightedGraph;
import java_utils.UnionFind;

public class _4_3_22 {

    public class ConnectedComponents {

        private boolean[] visited;
        private int[] id;
        private int count;
    
        public ConnectedComponents(EdgeWeightedGraph graph) {
            visited = new boolean[graph.vertices()];
            id = new int[graph.vertices()];
    
            for (int source = 0; source < graph.vertices(); source++) {
                if (!visited[source]) {
                    dfs(graph, source);
                    count++;
                }
            }
        }
    
        private void dfs(EdgeWeightedGraph graph, int vertex) {
            visited[vertex] = true;
            id[vertex] = count;
    
            for (Edge edge : graph.adjacent(vertex)) {
                if (!visited[edge.other(vertex)]) {
                    dfs(graph, edge.other(vertex));
                }
            }
        }
    
        public boolean connected(int vertex1, int vertex2) {
            return id[vertex1] == id[vertex2];
        }
    
        public int id(int vertex) {
            return id[vertex];
        }
    
        public int count() {
            return count;
        }
    }

    public class LazyPrimMST {

        private boolean[] marked; // minimum spanning tree vertices
        private Queue<Edge> minimumSpanningTree;
        private PriorityQueue<Edge> priorityQueue; // crossing (and ineligible) edges
        private double weight;
    
        public LazyPrimMST(EdgeWeightedGraph edgeWeightedGraph) {
            priorityQueue = new PriorityQueue<>(PriorityQueue.Orientation.MIN);
            marked = new boolean[edgeWeightedGraph.vertices()];
            minimumSpanningTree = new Queue<>();
    
            // Find connected components
            ConnectedComponents connectedComponents = new ConnectedComponents(edgeWeightedGraph);

            // Iterate through connected comoponents
            for (int connectedComponentID = 0; connectedComponentID < connectedComponents.count(); connectedComponentID++) {
                // Find a vertex in the given connectedComponentID.
                int initialVertex = 0;
                while (connectedComponents.id(initialVertex) != connectedComponentID) {
                    initialVertex += 1;
                }

                // Start from initialVertex
                visit(edgeWeightedGraph, initialVertex); // assumes the graph is connected
    
                while (!priorityQueue.isEmpty()) {
                    Edge edge = priorityQueue.deleteTop(); // Get lowest-weight edge from priority queue
                    int vertex1 = edge.either();
                    int vertex2 = edge.other(vertex1);
        
                    // Skip if ineligible
                    if (marked[vertex1] && marked[vertex2]) {
                        continue;
                    }
        
                    // Add edge to the minimum spanning tree
                    minimumSpanningTree.enqueue(edge);
                    weight += edge.weight();
        
                    // Add vertex to the minimum spanning tree
                    if (!marked[vertex1]) {
                        visit(edgeWeightedGraph, vertex1);
                    }
                    if (!marked[vertex2]) {
                        visit(edgeWeightedGraph, vertex2);
                    }
                }
            }
        }
    
        private void visit(EdgeWeightedGraph edgeWeightedGraph, int vertex) {
            // Mark vertex and add to priority queue all edges from vertex to unmarked vertices
            marked[vertex] = true;
    
            for (Edge edge : edgeWeightedGraph.adjacent(vertex)) {
                if (!marked[edge.other(vertex)]) {
                    priorityQueue.insert(edge);
                }
            }
        }
    
        public Iterable<Edge> edges() {
            return minimumSpanningTree;
        }
    
        public double lazyWeight() {
            double weight = 0;
    
            for (Edge edge : edges()) {
                weight += edge.weight();
            }
            return weight;
        }
    
        public double eagerWeight() {
            return weight;
        }
    }

    // Given KruskalMST adds the lowest-weight eligible edge one at a time to the MST, don't think it matters whether edge-weighted graph is connected
    public class KruskalMST {

        private Queue<Edge> minimumSpanningTree;
        private double weight;
    
        public KruskalMST(EdgeWeightedGraph edgeWeightedGraph) {
            minimumSpanningTree = new Queue<>();
            PriorityQueue<Edge> priorityQueue = new PriorityQueue<>(PriorityQueue.Orientation.MIN);
    
            for (Edge edge : edgeWeightedGraph.edges()) {
                priorityQueue.insert(edge);
            }
    
            UnionFind unionFind = new UnionFind(edgeWeightedGraph.vertices());
    
            while (!priorityQueue.isEmpty() && minimumSpanningTree.size() < edgeWeightedGraph.vertices() - 1) {
                Edge edge = priorityQueue.deleteTop(); // Get lowest-weight edge from priority queue
                int vertex1 = edge.either();
                int vertex2 = edge.other(vertex1);
    
                // Ignore ineligible edges
                if (unionFind.connected(vertex1, vertex2)) {
                    continue;
                }
    
                unionFind.union(vertex1, vertex2);
                minimumSpanningTree.enqueue(edge); // Add edge to the minimum spanning tree
    
                weight += edge.weight();
            }
        }
    
        public Iterable<Edge> edges() {
            return minimumSpanningTree;
        }
    
        public double lazyWeight() {
            double weight = 0;
    
            for (Edge edge : edges()) {
                weight += edge.weight();
            }
    
            return weight;
        }
    
        public double eagerWeight() {
            return weight;
        }
    }

    public static void main(String[] args) {
        _4_3_22 exercise = new _4_3_22();

        // Edge-weighted graph from p605
        EdgeWeightedGraph graph = new EdgeWeightedGraph(7);
        graph.addEdge(new Edge(4, 5, 0.61));
        graph.addEdge(new Edge(4, 6, 0.62));
        graph.addEdge(new Edge(5, 6, 0.88));
        graph.addEdge(new Edge(1, 5, 0.11));
        graph.addEdge(new Edge(2, 3, 0.35));
        graph.addEdge(new Edge(0, 3, 0.6));
        graph.addEdge(new Edge(1, 6, 0.10));
        graph.addEdge(new Edge(0, 2, 0.22));
        
        ConnectedComponents connectedComponents = exercise.new ConnectedComponents(graph);
        System.out.println(connectedComponents.count());
        LazyPrimMST mst1 = exercise.new LazyPrimMST(graph);

        for (Edge edge : mst1.edges()) {
            System.out.println(edge.either() + " - " + edge.other(edge.either()) + " : " + edge.weight());
        }

        System.out.println("---------");

        KruskalMST mst2 = exercise.new KruskalMST(graph);
        for (Edge edge : mst2.edges()) {
            System.out.println(edge.either() + " - " + edge.other(edge.either()) + " : " + edge.weight());
        }
    }
}
