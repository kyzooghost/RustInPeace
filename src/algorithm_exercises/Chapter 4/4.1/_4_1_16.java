// 4.1.16 - p559

// Eccentricity(v) - length of shortest path from that vertex, to the furthest vertex from v
// Diameter - Maximum eccentricity
// Radius - Smallest eccentricity
// Center - Vertex whose eccentricity = radius

import java_utils.Graph;
import java_utils.Queue;
import java_utils.Stack;

public class _4_1_16 {
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

        public GraphProperties(Graph graph) throws Exception {
            // Check if Graph connected
            BreadthFirstPaths bfs_test = new BreadthFirstPaths(graph, 0);
            for (boolean mark : bfs_test.marked) {
                if (!mark) {throw new Exception("Exception message");}
            }

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
        _4_1_16 exercise = new _4_1_16();
        Graph graph = new Graph(12);
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

        try {
            GraphProperties gp = exercise.new GraphProperties(graph);
            System.out.println("No Error!");
        } catch (Exception e) {
            System.out.println("Error!");
        }

        // 0 => 0
        // 5 => 1
        // 2 => 2
        // 3 => 3
        // 6 => 4
        // 10 => 5
        Graph graph1 = new Graph(6);
        graph1.addEdge(0, 1);
        graph1.addEdge(0, 2);
        graph1.addEdge(0, 4);
        graph1.addEdge(2, 4);
        graph1.addEdge(2, 1);
        graph1.addEdge(2, 3);
        graph1.addEdge(1, 5);
        graph1.addEdge(3, 5);

        try {
            GraphProperties gp = exercise.new GraphProperties(graph1);
            System.out.println("No Error!");
            System.out.println("Eccentricity of 0: " + gp.eccentricity(0));
            System.out.println("Eccentricity of 1: " + gp.eccentricity(1));
            System.out.println("Eccentricity of 2: " + gp.eccentricity(2));
            System.out.println("Eccentricity of 3: " + gp.eccentricity(3));
            System.out.println("Eccentricity of 4: " + gp.eccentricity(4));
            System.out.println("Eccentricity of 5: " + gp.eccentricity(5));
            System.out.println("Diameter: " + gp.diameter());
            System.out.println("Radius: " + gp.radius());
            System.out.println("Center: " + gp.center());
        } catch (Exception e) {
            System.out.println("Error!");
        }

    }
}
