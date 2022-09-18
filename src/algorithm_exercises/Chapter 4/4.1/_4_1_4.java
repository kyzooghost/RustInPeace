// 4.1.4

// Add a method hasEdge() to Graph which takes two int arguments v and w and returns true if the graph has an edge v-w, false otherwise.

import java_utils.GraphInterface;
import java_utils.Bag;

@SuppressWarnings("unchecked")
public class _4_1_4 {
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

    public static void main(String[] args) {
        _4_1_4 exercise = new _4_1_4();
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
        System.out.println("should be true: " + graph.hasEdge(0, 5));
        System.out.println("should be false: " + graph.hasEdge(9, 5));
    }
}


