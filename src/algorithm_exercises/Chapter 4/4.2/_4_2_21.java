// LCA of a DAG. 

// Lowest common ancestor (LCA) of two vertices (v and w)
// Given DAG
// LCA has no children, that are also parents of v and w

// LCA => Multiple inheritance in programming languages, analysis of genealogical data
// Hint - Define height of vertex v in a DAG, to be the length of the longest path from a root to v.
// Greatest height common ancestor => LCA

// Find all the sources (indegree == 0)
// Using DepthFirstPaths from each source, mark the height of each vertix

// To find LCA - Reverse graph, do BFS from each vertix and find if share parent
// Check height of each shared parent

import java_utils.Digraph;
import java_utils.Queue;
import java_utils.Stack;
import java_utils.DirectedCycle;
import java_utils.GraphInterface;
import java.util.ArrayList;
import java.util.Iterator;

@SuppressWarnings("unchecked")
public class _4_2_21 {
    public class LCA {
        Digraph graph;
        int[] heights;

        public LCA (Digraph graph_) throws IllegalArgumentException {
            graph = graph_;

            // INPUT VALIDATION - check that graph does not have a directed cycle
            DirectedCycle cycleCheck = new DirectedCycle(graph_);
            if (cycleCheck.hasCycle()) throw new IllegalArgumentException("Cannot find LCA of a directed cycle");

            heights = new int[graph.vertices()];
            for (int i = 0; i < heights.length; i++) {heights[i] = -1;}
            ArrayList<Integer> sources = new ArrayList<Integer>();
            Iterator<Integer>[] adjacent = new Iterator[graph.vertices()];

            // 1. COLLECT THE SOURCES and CREATE LOCAL ITERATOR INSTANCES OF ADJACENCY LISTS, FOR ITERATIVE DFS
            for (int i = 0; i < graph.vertices(); i++) {
                if (graph.indegree(i) == 0) {sources.add(i);}
                adjacent[i] = graph.adjacent(i).iterator();
            }

            // 2. UNMARKED ITERATIVE DFS FROM EACH SOURCE, FIND HEIGHTS
            Stack<Integer> dfsStack = new Stack<>();

            while (sources.size() > 0) {
                int source = sources.get(0);
                sources.remove(0); // Unsure if can keep popping from 0 index

                dfsStack.push(source); // Push onto DFS stack until we can no longer find something

                while (!dfsStack.isEmpty()) {
                    int vertex = dfsStack.peek();

                    while (adjacent[vertex].hasNext()) {
                        vertex = adjacent[vertex].next(); // Consume next item of iterator + reassign `vertex` to adjacent vertex
                        dfsStack.push(vertex); // Push onto dfs stack, then scan adjacency list of neighbour
                        // Loop ends when DFS hits 'dead-end'
                    }

                    dfsStack.pop();
                    heights[vertex] = Math.max(dfsStack.size(), heights[vertex]); // Height is DFS stack size after element popped
                }
            }
        }

        public int findLCA(int v1, int v2) {
            Digraph reverse = graph.reverse();

            // BFS from each vertex (in reversed digraph) to find parents
            ArrayList<Integer> v1_parents = bfs(reverse, v1);
            ArrayList<Integer> v2_parents = bfs(reverse, v2);
            ArrayList<Integer> common_parents = new ArrayList<Integer>();

            // Find common parents
            for (int i : v1_parents) {
                if (v2_parents.contains(i)) {common_parents.add(i);}
            }
            
            // No common parents
            if (common_parents.isEmpty()) {
                return -1;
            // Else find common parent with greatest height
            } else {
                int lca = common_parents.get(0);
                int max_height = heights[lca];

                for (int i : common_parents) {
                    if (max_height < heights[i]) {
                        max_height = heights[i];
                        lca = i;
                    }
                }

                return lca;
            }
        }

        private ArrayList<Integer> bfs(GraphInterface graph, int sourceVertex) {
            ArrayList<Integer> visited = new ArrayList<Integer>();
            Queue<Integer> queue = new Queue<>();
            visited.add(sourceVertex);
            queue.enqueue(sourceVertex);
    
            while (!queue.isEmpty()) {
                int currentVertex = queue.dequeue();
    
                for (int neighbor : graph.adjacent(currentVertex)) {
                    if (!visited.contains(neighbor)) {
                        visited.add(neighbor);
                        queue.enqueue(neighbor);
                    }
                }
            }

            return visited;
        }

        public void printHeights() {
            for (int i = 0; i < heights.length; i++) {
                System.out.println(i + " - " + heights[i]);
            }
        }
    }

    public static void main(String[] args) {
        _4_2_21 exercise = new _4_2_21();
        // https://i.stack.imgur.com/ESLxH.png
        Digraph graph = new Digraph(10);
        graph.addEdge(0, 1);
        graph.addEdge(0, 2);
        graph.addEdge(1, 3);
        graph.addEdge(1, 4);
        graph.addEdge(2, 3);
        graph.addEdge(2, 4);
        graph.addEdge(2, 5);
        graph.addEdge(2, 6);
        graph.addEdge(5, 8);
        graph.addEdge(6, 8);
        graph.addEdge(6, 7);
        graph.addEdge(6, 9);
        graph.addEdge(7, 9);

        LCA lca = exercise.new LCA(graph);
        System.out.println(lca.findLCA(1, 6));
        // lca.printHeights();

    }
}
