/*
 * Adapt the DirectedCycle and Topological classes from Section 4.2 to use the EdgeweightedDigraph and DirectedEdge APIs of this section, thus implementing EdgeWeightedCycleFinder and EdgeWeightedTopological classes.
 */

import java_utils.Stack;
import java_utils.Queue;
import java_utils.DirectedEdge;
import java_utils.EdgeWeightedDigraph;

public class _4_4_12 {
    public class DirectedCycle {
        private boolean[] marked;
        private int[] edgeTo;
        private Stack<Integer> cycle; // Vertices on a cycle
        private boolean[] onStack; // Vertices on recursive call stack
    
        // DFS to find if cycle exists
        public DirectedCycle(EdgeWeightedDigraph G) {
            onStack = new boolean[G.vertices()];
            edgeTo = new int[G.vertices()];
            marked = new boolean[G.vertices()];
            for (int v = 0; v < G.vertices(); v++) {
                if (!marked[v]) dfs(G, v);
            }
        }
    
        private void dfs(EdgeWeightedDigraph G, int v) {
            onStack[v] = true;
            marked[v] = true;
    
            // Iterate through each neighbor
            for (DirectedEdge e : G.adjacent(v)) {
                if (this.hasCycle()) return;
                else if (!marked[e.to()]) {
                    edgeTo[e.to()] = v; // Mark first DFS path found to w
                    dfs(G, e.to());
                }
                // Found neighbor which already on DFS stack => cycle found!
                else if (onStack[e.to()]) {
                    cycle = new Stack<Integer>();
                    // Cycle starts from v, fill in path 'backwards' using edgeTo
                    for (int x = v; x != e.to(); x = edgeTo[x]) {
                        cycle.push(x);
                    }
                    // Add "w" and "v" as cherry on top to complete cycle
                    cycle.push(e.to());
                    cycle.push(v);
                }
            }
    
            onStack[v] = false;
        }
    
        public boolean hasCycle() {return cycle != null;}
        public Iterable<Integer> cycle() {return cycle;}
    }

    public class DepthFirstOrder {
        private boolean[] marked;
        private Queue<Integer> pre; // vertices in preorder (order of DFS call made)
        private Queue<Integer> post; // vertices in postorder (order of DFS call completed)
        private Stack<Integer> reversePost; // vertices in reverse postorder (also topological sort)
    
        public DepthFirstOrder(EdgeWeightedDigraph G) {
            pre = new Queue<Integer>();
            post = new Queue<Integer>();
            reversePost = new Stack<Integer>();
            marked = new boolean[G.vertices()];
    
            for (int v = 0; v < G.vertices(); v++) {
                if (!marked[v]) dfs(G, v);
            }
        }
    
        private void dfs(EdgeWeightedDigraph G, int v) {
            pre.enqueue(v);
            for (DirectedEdge e: G.adjacent(v)) {
                if (!marked[e.to()]) dfs(G, e.to());
            }
            post.enqueue(v);
            reversePost.push(v);
        }
    
        public Iterable<Integer> pre() {return pre;}
        public Iterable<Integer> post() {return post;}
        public Iterable<Integer> reversePost() {return reversePost;}
    }

    public class Topological {
        private Iterable<Integer> order;
    
        public Topological(EdgeWeightedDigraph G) {
            // First DFS to check for cycle
            DirectedCycle cyclefinder = new DirectedCycle(G);
    
            // Second DFS to find reverse post order
            if (!cyclefinder.hasCycle()) {
                DepthFirstOrder dfs = new DepthFirstOrder(G);
                order = dfs.reversePost(); // Reverse post order == topological sort
            }
        }
    
        public Iterable<Integer> order() {return order;}
        public boolean isDAG() {return order == null;}
        public void printOrder() {
            StringBuilder string = new StringBuilder();
            for (int i : order) {
                string.append(i).append(" ");
            }
            System.out.println(string);
        }
    }

    public static void main(String[] args) {
    }
}
