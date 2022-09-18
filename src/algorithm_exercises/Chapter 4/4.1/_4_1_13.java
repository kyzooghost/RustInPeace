// 4.1.13 - p559

// Add a distTo() method to the BreadthFirstPaths API and implementation, which returns the number of edges on the shortest path from the source to a given vertex. A distTo() query should run in constant time.

// p540 - BreadthFirstPaths

import java_utils.Graph;
import java_utils.Queue;
import java_utils.Stack;

@SuppressWarnings("unchecked")
public class _4_1_13 {

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
  

    public static void main(String[] args) {
        _4_1_13 exercise = new _4_1_13();
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

        BreadthFirstPaths query = exercise.new BreadthFirstPaths(graph, 0);
        System.out.println("hasPath: " + query.hasPathTo(10));
        System.out.println("hasPath: " + query.hasPathTo(9));

        StringBuilder stringBuilder = new StringBuilder();
        Iterable<Integer> path = query.pathTo(3);
        for (int i : path) {
            stringBuilder.append(i).append("-");
        }
        stringBuilder.deleteCharAt(stringBuilder.length() - 1);
        System.out.println(stringBuilder);

        System.out.println("distTo 3: " + query.distTo(3));
        System.out.println("distTo 5: " + query.distTo(5));
    }
}