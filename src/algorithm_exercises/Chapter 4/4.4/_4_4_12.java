/*
 * Adapt the DirectedCycle and Topological classes from Section 4.2 to use the EdgeweightedDigraph and DirectedEdge APIs of this section, thus implementing EdgeWeightedCycleFinder and EdgeWeightedTopological classes.
 */

import java_utils.Stack;
import java_utils.DirectedEdge;

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
            for (int w : G.adjacent(v)) {
                if (this.hasCycle()) return;
                else if (!marked[w]) {
                    edgeTo[w] = v; // Mark first DFS path found to w
                    dfs(G, w);
                }
                // Found neighbor which already on DFS stack => cycle found!
                else if (onStack[w]) {
                    cycle = new Stack<Integer>();
                    // Cycle starts from v, fill in path 'backwards' using edgeTo
                    for (int x = v; x != w; x = edgeTo[x]) {
                        cycle.push(x);
                    }
                    // Add "w" and "v" as cherry on top to complete cycle
                    cycle.push(w);
                    cycle.push(v);
                }
            }
    
            onStack[v] = false;
        }
    
        public boolean hasCycle() {return cycle != null;}
        public Iterable<Integer> cycle() {return cycle;}
    }
}
