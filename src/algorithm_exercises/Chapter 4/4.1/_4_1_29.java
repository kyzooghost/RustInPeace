// 561

// Modify Cycle so that it works even if the graph contains self-loops and parallel edges.

import java_utils.Graph;

public class _4_1_29 {
    public class Cycle {
        private boolean[] visited;
        private boolean hasCycle;
    
        public Cycle(Graph graph) {
            visited = new boolean[graph.vertices()];
    
            for (int source = 0; source < graph.vertices(); source++) {
                if (!visited[source]) {
                    dfs(graph, source, source);
                }
            }
        }
    
        private void dfs(Graph graph, int vertex, int origin) {
            visited[vertex] = true;
    
            // Create a set from graph.adjacent(vertex), and remove vertex from it, then iterate through this set.
            // Set => remove duplicates => remove parallel edges?

            // Iterate through each neighbour of vertex
            for (int neighbor : graph.adjacent(vertex)) {
                if (neighbor == vertex) {continue;} // Skip self loop
                if (!visited[neighbor]) {
                    dfs(graph, neighbor, vertex);
                // How to skip parallel edge here?
                } else if (neighbor != origin) {
                    hasCycle = true;
                }
            }
        }
    
        public boolean hasCycle() {
            return hasCycle;
        }
    }
}
