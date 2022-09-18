package java_utils;

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
