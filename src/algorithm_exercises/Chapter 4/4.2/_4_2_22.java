// Shortest ancestral path. Given a DAG and two vertices v and w, find the shortest ancestral path between v and w. An ancestral path between v and w is a common ancestor x along with a shortest path from v to x and a shortest path from w to x. The shortest ancestral path is the ancestral path whose total length is minimized. 

// Warmup: Find a DAG where the shortest ancestral path goes to a common ancestor x that is not an LCA. Hint: Run BFS twice, once from v and once from w.

// Shortest ancestral path

// Given DAG, and two vertices v1 and v2
// Find shortest ancestral path - common ancestor x + shortest path from v1 to x, and shortest path from v2 to x

// Run BFS on reverse DAG, using v1 and v2 as source
// Find common ancestor such that distance minimized
// Print the path

import java_utils.Digraph;
import java_utils.Queue;
import java_utils.Stack;
import java_utils.DirectedCycle;
import java_utils.GraphInterface;
import java_utils.BreadthFirstPaths;
import java.util.ArrayList;
import java.util.Iterator;

@SuppressWarnings("unchecked")
public class _4_2_22 {
    public class FindSAP {
        private Digraph graph;

        public FindSAP (Digraph graph_) throws IllegalArgumentException {
            graph = graph_;

            // INPUT VALIDATION - check that graph does not have a directed cycle
            DirectedCycle cycleCheck = new DirectedCycle(graph_);
            if (cycleCheck.hasCycle()) throw new IllegalArgumentException("Cannot find LCA of a directed cycle");
        }

        public ArrayList<Integer> findShortestAncestralPath(int v1, int v2) {
            Digraph reverse = graph.reverse();
            ArrayList<Integer> path = new ArrayList<>();

            // BFS from each vertex (in reversed digraph).
            BreadthFirstPaths bfs1 = new BreadthFirstPaths(reverse, v1);
            BreadthFirstPaths bfs2 = new BreadthFirstPaths(reverse, v2);

            // Find common parents with shortest distance
            int common_parent = -1;
            int min_length = Integer.MAX_VALUE;

            for (int i = 0; i < graph.vertices(); i++) {
                if (i != v1 && i != v2 && bfs1.hasPathTo(i) && bfs2.hasPathTo(i)) {
                    if (min_length > bfs1.distTo(i) + bfs2.distTo(i)) {
                        min_length = bfs1.distTo(i) + bfs2.distTo(i);
                        common_parent = i;
                    }
                } 
            }

            if (common_parent == -1) return path;

            // Collect path
            Queue<Integer> v1_to_ancestor_path = new Queue<Integer>();
            Stack<Integer> ancestor_to_v2_path = new Stack<Integer>();

            for (int i : bfs1.pathTo(common_parent)) {v1_to_ancestor_path.enqueue(i);}
            for (int i : bfs2.pathTo(common_parent)) {ancestor_to_v2_path.push(i);}
            
            while (!v1_to_ancestor_path.isEmpty()) {path.add(v1_to_ancestor_path.dequeue());}
            while (!ancestor_to_v2_path.isEmpty()) {path.add(ancestor_to_v2_path.pop());}
            return path;
        }
    }

    public static void main(String[] args) {
        _4_2_22 exercise = new _4_2_22();
        // https://i.stack.imgur.com/ESLxH.png
        Digraph graph = new Digraph(9);
        graph.addEdge(0, 1);
        graph.addEdge(1, 3);
        graph.addEdge(1, 4);
        graph.addEdge(4, 5);
        graph.addEdge(5, 6);
        graph.addEdge(6, 2);
        graph.addEdge(7, 8);
        graph.addEdge(8, 3);
        graph.addEdge(7, 2);

        FindSAP query = exercise.new FindSAP(graph);
        Iterable<Integer> path = query.findShortestAncestralPath(2, 3);

        StringBuilder string = new StringBuilder();
        for (int i : path) {string.append(i).append(" - ");}
        string.deleteCharAt(string.length() - 1);
        string.deleteCharAt(string.length() - 1);
        System.out.println(string);
    }
}