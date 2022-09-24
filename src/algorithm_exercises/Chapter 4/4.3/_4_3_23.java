// Vyssotsky’s algorithm. 

// Develop an implementation that computes the MST by applying the cycle property (see Exercise 4.3.8) repeatedly: Add edges one at a time to a putative tree, deleting a maximum-weight edge on the cycle if one is formed. 

// Note : This method has received less attention than the others that we consider because of the comparative difficulty of maintaining a data structure that supports efficient implementation of the “delete the maximum-weight edge on the cycle’’ operation.

// Cycle property - Given any cycle in an edge-weighted graph, the edge of the maximum weight does not belong in the MST to the graph

// Add edge one at a time
// Check what cycles created by new edge
// Delete max weight on each new cycle
// MST will not have cycles

// We need to support deleting an arbitrary edge
// HashMap (key = Edge vertices.toString(), value = ArrayList<Edge> index)
// BST <key = index, value = Edge>
// No self-edge
// If parallel edge, put lower weight into BST

// We create a cycle if both vertices are visited already

// Time complexity of my current solution is O(E^2 lg N), vs Kruskal's and Prim's time complexity of O(E lg E)

import java_utils.EdgeWeightedGraph;
import java_utils.Edge;
import java_utils.SeparateChainingHashTable;
import java_utils.PriorityQueue.Orientation;
import java_utils.RedBlackBST;
import java_utils.PriorityQueue;
import java_utils.Queue;

public class _4_3_23 {
    public class VysMST {
        private double weight;
        private boolean[] visited;
        private SeparateChainingHashTable<String, Integer> edgeToIndexMap;
        private RedBlackBST<Integer, Edge> indexToEdgeMap;
        private Queue<Edge> mst;

        public VysMST (EdgeWeightedGraph graph) {
            weight = 0;
            visited = new boolean[graph.vertices()];
            edgeToIndexMap = new SeparateChainingHashTable<String, Integer>();
            indexToEdgeMap = new RedBlackBST<Integer, Edge>();
            mst = new Queue<Edge>();

            // Iterate through each edge and apply Vyssotsky’s algorithm - O (E^2 * lg N) - Nested for-loop for priority queue insertion
            int index = 0;

            for (Edge edge : graph.edges()) {
                int vertex1 = edge.either();
                int vertex2 = edge.other(vertex1);

                // Ignore self loop
                if (vertex1 == vertex2) {continue;}

                // If parallel edge, only keep smaller edge
                if (edgeToIndexMap.contains(_edgeToString(edge))) {
                    int currentEdgeIndex = edgeToIndexMap.get(_edgeToString(edge));
                    double currentEdgeWeight = indexToEdgeMap.get(currentEdgeIndex).weight();
                    if (edge.weight() < currentEdgeWeight) {indexToEdgeMap.put(currentEdgeIndex, edge);} // Replace if smaller key
                    continue;
                }

                // Else if new edge, first add to symbol tables
                edgeToIndexMap.put(_edgeToString(edge), index); // O(1) amortized
                indexToEdgeMap.put(index, edge); // O(lg N)
                index += 1;

                // Then check if cycle is made => Edge connects two already visited vertices
                if (visited[vertex1] && visited[vertex2]) {
                    // Find created cycle and to a maximum priority queue
                    Iterable<Edge> cyclePath = _findNewCycle(edge); // BFS so O(E + V)
                    
                    // Find max-weight edge in cycle and remove from symbol tables
                    PriorityQueue<Edge> maxWeightEdgeGetter = new PriorityQueue<Edge>(Orientation.MAX);

                    StringBuilder string2 = new StringBuilder();
                    for (Edge e : cyclePath) {
                        maxWeightEdgeGetter.insert(e);
                    } // O (E lg E => E * Priority-queue insert)
                    Edge maxWeightEdge = maxWeightEdgeGetter.deleteTop();
                    int maxWeightEdgeIndex = edgeToIndexMap.get(_edgeToString(maxWeightEdge));
                    edgeToIndexMap.delete(_edgeToString(maxWeightEdge));
                    indexToEdgeMap.delete(maxWeightEdgeIndex);
                    continue;
                }

                visited[vertex1] = true;
                visited[vertex2] = true;
            }

            // Now only MST remains in symbol tables, iterate through symbol tables to get MST
            for (int STindex : indexToEdgeMap.keys()) {
                weight += indexToEdgeMap.get(STindex).weight();
                mst.enqueue(indexToEdgeMap.get(STindex));
            }
        }

        private Iterable<Edge> _findNewCycle(Edge edge_) {
            int vertex1 = edge_.either();
            int vertex2 = edge_.other(vertex1);
            Queue<Integer> bfsQueue = new Queue<Integer>();
            Edge[] edgeTo = new Edge[visited.length];
            boolean[] bfsVisited = new boolean[visited.length];
            edgeTo[vertex1] = null; // If we are doing BFS from vertex1, and there is no cycle, we cannot have an edge to vertex1

            // Create graph with current edges, except most recently added
            EdgeWeightedGraph graph = new EdgeWeightedGraph(visited.length);
            int mostRecentlyAddedIndex = edgeToIndexMap.get(_edgeToString(edge_));
            for (int index : indexToEdgeMap.keys()) {
                if (index != mostRecentlyAddedIndex) graph.addEdge(indexToEdgeMap.get(index));
            }

            // BFS starting from vertex1, to fill edgeTo[] array
            bfsQueue.enqueue(vertex1);

            while (!bfsQueue.isEmpty()) {
                int vertex = bfsQueue.dequeue();
                bfsVisited[vertex] = true;

                for (Edge e: graph.adjacent(vertex)) {
                    // Skip if visited (we store duplicates of each edge in EdgeWeightedGraph implementation)
                    if (bfsVisited[e.other(vertex)]) {continue;}
                    int otherVertex = e.other(vertex);
                    edgeTo[otherVertex] = e;
                    bfsQueue.enqueue(otherVertex);
                }
            }

            // Iterate backwards from vertex2 in edgeTo[] to find the max-weighted edge in path from vertex1 to vertex2
            Queue<Edge> cyclePath = new Queue<Edge>();
            int pointerVertex = vertex2;

            while (pointerVertex != vertex1) {
                Edge e = edgeTo[pointerVertex];
                cyclePath.enqueue(e);
                pointerVertex = e.other(pointerVertex);
            }

            cyclePath.enqueue(edge_);
            return cyclePath;
        }

        private String _edgeToString(Edge edge) {
            int vertex1 = edge.either();
            int vertex2 = edge.other(vertex1);
            if (vertex1 < vertex2) {return String.format("%d-%d", vertex1, vertex2);}
            else {return String.format("%d-%d", vertex2, vertex1);}
        }

        public Iterable<Edge> edges() {
            return mst;
        }
    }

    public static void main(String[] args) {
        _4_3_23 exercise = new _4_3_23();

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

        VysMST mst1 = exercise.new VysMST(graph);

        for (Edge edge : mst1.edges()) {
            System.out.println(edge.either() + " - " + edge.other(edge.either()) + " : " + edge.weight());
        }
    }
}
