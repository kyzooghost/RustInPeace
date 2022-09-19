// Symbol graph. Implement a one-pass SymbolGraph (it need not be a Graph client). Your implementation may pay an extra log V factor for graph operations, for symbol-table lookups.

import java_utils.SeparateChainingHashTable;
import java_utils.Graph;
import java_utils.In;
import java.util.ArrayList;

public class _4_1_34 {
    public class SymbolGraph {

        private SeparateChainingHashTable<String, Integer> vertexNameToIdMap;
        private String[] keys;
        private Graph graph;
    
        public SymbolGraph(String stream, String separator) {
            vertexNameToIdMap = new SeparateChainingHashTable<>();
    
            // FIRST PASS
            In in = new In(stream);
            Graph graph1 = new Graph(); // Modified to not require providing integer at start, store each edges in Symbol Table lmao

            while (in.hasNextLine()) {
                String[] vertices = in.readLine().split(separator);

                // Iterate through entries, and insert into "name => ID" symbol tree
                for (int i = 0; i < vertices.length; i++) {
                    if (!vertexNameToIdMap.contains(vertices[i])) {
                        vertexNameToIdMap.put(vertices[i], vertexNameToIdMap.size());
                    }
                }

                int movieVertexID = vertexNameToIdMap.get(vertices[0]);
                for (int i = 0; i < vertices.length; i++) {
                    graph.addEdge(movieVertexID, vertexNameToIdMap.get(vertices[i]));
                }                
            }
    
            // Create reverse index "ID => name" using String array
            keys = new String[vertexNameToIdMap.size()];
    
            for (String vertexName : vertexNameToIdMap.keys()) {
                keys[vertexNameToIdMap.get(vertexName)] = vertexName;
            }

            // SECOND PASS

            // Can store movies, but how to store actors associated with each movie?
            // Symbol table with multiple values for each key
            // Iterate through each key

            // Create Graph
            graph = new Graph(vertexNameToIdMap.size());
            in = new In(stream);
    
            // Iterate through each entry
            while (in.hasNextLine()) {
                String[] vertices = in.readLine().split(separator);
                // Get ID of movie
                int vertex = vertexNameToIdMap.get(vertices[0]);

                // Add edge between movie ID, and actor IDs
                // Data format is Movie/Actor/Actor/Actor...
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
    
}
