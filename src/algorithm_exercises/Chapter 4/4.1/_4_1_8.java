// 4.1.8 - p558

// Develop an implementation for the SearchAPI on page 528 that uses UF, as described in the text.

// p219 - UF interface
// p228 - Weighted quick UF implementation - balancing tree of nodes

import java_utils.GraphInterface;
import java_utils.Bag;
import java_utils.WeightedQuickUnionUF;

@SuppressWarnings("unchecked")
public class _4_1_8 {
    public class Graph implements GraphInterface {

        private final int vertices;
        protected int edges;
        private Bag<Integer>[] adjacent;
    
        public Graph(int vertices) {
            this.vertices = vertices;
            this.edges = 0;
            adjacent = (Bag<Integer>[]) new Bag[vertices];
    
            for (int vertex = 0; vertex < vertices; vertex++) {
                adjacent[vertex] = new Bag<>();
            }
        }
    
        public int vertices() {
            return vertices;
        }
    
        public int edges() {
            return edges;
        }
    
        public void addEdge(int vertex1, int vertex2) {
            // If parallel edge, return
            for (int neighbor : adjacent(vertex1)) {
                if (neighbor == vertex2) {return;}
            }

            // If self-loop, return
            if (vertex1 == vertex2) {return;}

            adjacent[vertex1].add(vertex2);
            adjacent[vertex2].add(vertex1);
            edges++;
        }
    
        public Bag<Integer>[] getAdjacencyList() {
            return adjacent;
        }
    
        public void updateAdjacencyList(int vertex, Bag adjacencyList) {
            adjacent[vertex] = adjacencyList;
        }
    
        public Iterable<Integer> adjacent(int vertex) {
            return adjacent[vertex];
        }
    
        public int degree(int vertex) {
            return adjacent[vertex].size();
        }
    
        @Override
        public String toString() {
            StringBuilder stringBuilder = new StringBuilder();
    
            for (int vertex = 0; vertex < vertices(); vertex++) {
                stringBuilder.append(vertex).append(": ");
    
                for (int neighbor : adjacent(vertex)) {
                    stringBuilder.append(neighbor).append(" ");
                }
                stringBuilder.append("\n");
            }
    
            return stringBuilder.toString();
        }

        public boolean hasEdge(int v, int w) {
            for (int neighbour : adjacent(v)) {
                if (neighbour == w) {return true;}
            }
            return false;
        }
    }

    public class Search {
        private final WeightedQuickUnionUF uf;
        private final int source_vertex;
        
        public Search(Graph graph, int source_vertex) {
            this.uf = new WeightedQuickUnionUF(graph.vertices());
            this.source_vertex = source_vertex;

            // Iterate through each vertex
            for (int vertex = 0; vertex < graph.vertices(); vertex++) {
                // Iterate through Bag of adjacent vertices, for each vertex
                for (int neighbour : graph.adjacent(vertex)) {
                    System.out.println(vertex + " --- " + neighbour);
                    this.uf.union(vertex, neighbour);
                }
            }
        }

        // Is vertex connected to source vertex?
        public boolean marked(int vertex) {
            return uf.connected(vertex, source_vertex);
        }

        // How many vertices are connected to source vertex?
        public int count() {
            return uf.treeSize(source_vertex) - 1;
        }
    }

    public static void main(String[] args) {
        _4_1_8 exercise = new _4_1_8();
        Graph graph = exercise.new Graph(12);
        graph.addEdge(8, 4);
        graph.addEdge(2, 3);
        graph.addEdge(1, 11);
        graph.addEdge(0, 6);
        graph.addEdge(3, 6);
        graph.addEdge(10, 3);
        graph.addEdge(7, 11);
        graph.addEdge(7, 8);
        graph.addEdge(11, 8);
        graph.addEdge(2, 0);
        graph.addEdge(6, 2);
        graph.addEdge(5, 2);
        graph.addEdge(5, 10);
        graph.addEdge(8, 1);
        graph.addEdge(4, 1);
        graph.addEdge(5, 0);

        Search search = exercise.new Search(graph, 0);
        System.out.println("count: " + search.count());
    }
}


