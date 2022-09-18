// p560

// Compute the number of connected components in movies.txt, the size of the largest component, and the number of components of size less than 10. 

// Find the eccentricity, diameter, radius, a center, and the girth of the largest component in the graph. 
// Does it contain Kevin Bacon?

import java_utils.Graph;
import java_utils.SeparateChainingHashTable;
import java_utils.In;
import java_utils.GraphInterface;
import java_utils.Stack;
import java_utils.Queue;

public class _4_1_24 {
    
    public class SymbolGraph {

        private SeparateChainingHashTable<String, Integer> vertexNameToIdMap;
        private String[] keys;
        private Graph graph;
    
        public SymbolGraph(String stream, String separator) {
            vertexNameToIdMap = new SeparateChainingHashTable<>();
    
            //First pass
            In in = new In(stream);
    
            while (in.hasNextLine()) {
                String[] vertices = in.readLine().split(separator);
    
                for (int i = 0; i < vertices.length; i++) {
                    if (!vertexNameToIdMap.contains(vertices[i])) {
                        vertexNameToIdMap.put(vertices[i], vertexNameToIdMap.size());
                    }
                }
            }
    
            keys = new String[vertexNameToIdMap.size()];
    
            for (String vertexName : vertexNameToIdMap.keys()) {
                keys[vertexNameToIdMap.get(vertexName)] = vertexName;
            }
    
            graph = new Graph(vertexNameToIdMap.size());
            //Seconds pass
            in = new In(stream);
    
            while (in.hasNextLine()) {
                String[] vertices = in.readLine().split(separator);
    
                int vertex = vertexNameToIdMap.get(vertices[0]);
                for (int i = 1; i < vertices.length; i++) {
                    graph.addEdge(vertex, vertexNameToIdMap.get(vertices[i]));
                }
            }
        }
    
        public boolean contains(String vertexName) {
            return vertexNameToIdMap.contains(vertexName);
        }
    
        public int index(String vertexName) {
            return vertexNameToIdMap.get(vertexName);
        }
    
        public String name(int vertexId) {
            return keys[vertexId];
        }
    
        public Graph graph() {
            return graph;
        }
    }

    public class ConnectedComponentsRecursiveDFS {

        private boolean[] visited;
        private int[] id;
        private int count;
        private int[] sizeOfComponents;
    
        public ConnectedComponentsRecursiveDFS(GraphInterface graph) {
            visited = new boolean[graph.vertices()];
            id = new int[graph.vertices()];
    
            for (int source = 0; source < graph.vertices(); source++) {
                if (!visited[source]) {
                    dfs(graph, source);
                    count++;
                }
            }

            sizeOfComponents = new int[count];
            for (int i : id) {sizeOfComponents[i] += 1;}
        }
    
        private void dfs(GraphInterface graph, int vertex) {
            visited[vertex] = true;
            id[vertex] = count;
    
            for (int neighbor : graph.adjacent(vertex)) {
                if (!visited[neighbor]) {
                    dfs(graph, neighbor);
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

        public int[] sizeOfComponents() {
            return sizeOfComponents;
        }
    }

    public class BreadthFirstPaths {
        private boolean[] marked; // Is a shortest path to this vertex known?
        private int[] edgeTo;     // last vertex on known path to this vertex
        private final int s;      // source

        public BreadthFirstPaths(Graph G, int s){
            marked = new boolean[G.vertices()];
            edgeTo = new int[G.vertices()];
            this.s = s;
            bfs(G, s);
        }
        
        private void bfs(Graph G, int s) {
            Queue<Integer> queue = new Queue<Integer>();
            marked[s] = true;          // Mark the source
            queue.enqueue(s);          //   and put it on the queue.
            while (!queue.isEmpty()) {
                int v = queue.dequeue(); // Remove next vertex from the queue.
                for (int w : G.adjacent(v)) {
                    if (!marked[w]) {
                        edgeTo[w] = v;
                        marked[w] = true;
                        queue.enqueue(w);
                    }
                }
            }
        }

        public boolean hasPathTo(int v) {return marked[v];}

        public Iterable<Integer> pathTo(int v) {
            if (!hasPathTo(v)) return null;
            Stack<Integer> path = new Stack<Integer>();
            for (int x = v; x != s; x = edgeTo[x])
               path.push(x);
            path.push(s);
            return path;
        }

        public int distTo(int v) {
            int result = 0;
            int x = v;

            while (x != s) {
                x = edgeTo[x]; // Doing this feels weird, especially after writing in Rust with explicit mutable and owned variables
                result += 1;
            }
            // Run in constant time
            return result;
        }
    }

    public class GraphProperties {
        private int[] eccentricity_array;

        public GraphProperties(Graph graph) {
            this.eccentricity_array = new int[graph.vertices()];

            // Find eccentricities
            for (int i = 0; i < graph.vertices(); i++) {
                eccentricity_array[i] = 0;

                BreadthFirstPaths bfs = new BreadthFirstPaths(graph, i);
                for (int j = 0; j < graph.vertices(); j++) {
                    if (i == j) {continue;}
                    if (bfs.hasPathTo(j) && bfs.distTo(j) > eccentricity_array[i]) {
                        eccentricity_array[i] = bfs.distTo(j);
                    }
                }
            }
        }

        public GraphProperties(Graph graph, int[] vertices) {
            this.eccentricity_array = new int[vertices.length];

            // Find eccentricities
            for (int i : vertices) {
                eccentricity_array[i] = 0;

                BreadthFirstPaths bfs = new BreadthFirstPaths(graph, i);
                for (int j : vertices) {
                    if (i == j) {continue;}
                    if (bfs.hasPathTo(j) && bfs.distTo(j) > eccentricity_array[i]) {
                        eccentricity_array[i] = bfs.distTo(j);
                    }
                }
            }
        }

        public int eccentricity(int v) {
            return eccentricity_array[v];
        }

        // Maximum eccentricity
        public int diameter() {
            int diameter = 0;
            for (int eccentricity : eccentricity_array) {
                if (eccentricity > diameter) {diameter = eccentricity;}
            }
            return diameter;
        }

        public int radius() {
            int radius = eccentricity_array[0];
            for (int eccentricity : eccentricity_array) {
                if (eccentricity < radius) {radius = eccentricity;}
            }
            return radius;
        }

        public int center() {
            int center = 0;
            for (int i = 0; i < eccentricity_array.length; i++) {
                if (eccentricity_array[i] == radius()) {center = i;}
            }
            return center;
        }
    }


    public static void main(String[] args) {
        _4_1_24 exercise = new _4_1_24();

        String FILE_PATH = "./movies.txt";
        String SEPARATOR = "/";
        SymbolGraph symbolGraph = exercise.new SymbolGraph(FILE_PATH, SEPARATOR);
        
        Graph graph = symbolGraph.graph();
        ConnectedComponentsRecursiveDFS connectionQuery = exercise.new ConnectedComponentsRecursiveDFS(graph);

        System.out.println("# of connected components: " + connectionQuery.count());

        int[] sizeOfComponents = connectionQuery.sizeOfComponents();
        int maxSize = 0;
        int maxSizedComponent = 0;
        int numComponents = 0;

        for (int i = 0; i < connectionQuery.count(); i++) {
            if (sizeOfComponents[i] > maxSize) {
                maxSize = sizeOfComponents[i];
                maxSizedComponent = i;
            }
            if (sizeOfComponents[i] < 10) {numComponents += 1;}
        }

        System.out.println("Size of biggest component: " + maxSize);
        System.out.println("Number of components of size less than 10: " + numComponents);

        int[] maxSizedComponentsVertices = new int[maxSize];
        int count = 0;

        for (int i = 0; i < symbolGraph.graph().vertices(); i++) {
            if (connectionQuery.id(i) == maxSizedComponent) {
                maxSizedComponentsVertices[count] = i;
                count += 1;
                if (count == maxSize) {break;}
            }
        }

        String kevinBaconName = "Bacon, Kevin";
        int kevinBaconId = symbolGraph.index(kevinBaconName);

        if (connectionQuery.id(kevinBaconId) == maxSizedComponent) {
            System.out.println("Largest component contains Kevin Bacon");
        } else {
            System.out.println("Largest component does not contain Kevin Bacon)");
        }

        // Use Kevin Bacon as center
        // Compute eccentricity of Kevin Bacon to get radius
        // Find vertices furthest from centre of the graph
        // Find diameter through finding eccentricities of vertices furthest from graph centre

        // GraphProperties graphProperties = exercise.new GraphProperties(graph, maxSizedComponentsVertices);
        // System.out.println("Eccentricity of largest component: " + graphProperties.diameter());
        // System.out.println("Eccentricity of largest component: " + graphProperties.radius());
        // System.out.println("Eccentricity of largest component: " + graphProperties.center());

        // Find the eccentricity, diameter, radius, a center, and the girth of the largest component in the graph. Does it contain Kevin Bacon?
    }

}
