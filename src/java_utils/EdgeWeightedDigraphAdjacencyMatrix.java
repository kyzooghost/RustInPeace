package java_utils;

public class EdgeWeightedDigraphAdjacencyMatrix {
    private final int vertices;
    private int edges;
    private double[][] adjacent;   // adjacency matrix that stores the edge weights
                                   // adjacent[i][j] = Double.POSITIVE_INFINITY if there is no direct edge
                                   // between vertex i and vertex j

    public EdgeWeightedDigraphAdjacencyMatrix(int vertices) {
        this.vertices = vertices;
        edges = 0;
        adjacent = new double[vertices][vertices];

        for (int vertex1 = 0; vertex1 < vertices; vertex1++) {
            for (int vertex2 = 0; vertex2 < vertices; vertex2++) {
                adjacent[vertex1][vertex2] = Double.POSITIVE_INFINITY;
            }
        }
    }

    public EdgeWeightedDigraphAdjacencyMatrix(In in) {
        this(in.readInt());
        int edges = in.readInt();

        if (edges < 0) {
            throw new IllegalArgumentException("Number of edges must be nonnegative");
        }

        for (int i = 0; i < edges; i++) {
            int vertexFrom = in.readInt();
            int vertexTo = in.readInt();
            double weight = in.readDouble();

            DirectedEdge edge = new DirectedEdge(vertexFrom, vertexTo, weight);
            addEdge(edge);
        }
    }

    public int vertices() {
        return vertices;
    }

    public int edgesCount() {
        return edges;
    }

    public void addEdge(DirectedEdge edge) {
        int fromVertex = edge.from();
        int toVertex = edge.to();

        // Parallel edges are ignored
        if (hasEdge(fromVertex, toVertex)) {
            return;
        }

        adjacent[fromVertex][toVertex] = edge.weight();
        edges++;
    }

    public boolean hasEdge(int vertex1, int vertex2) {
        return adjacent[vertex1][vertex2] != Double.POSITIVE_INFINITY;
    }

    public Iterable<DirectedEdge> adjacent(int vertex) {
        Bag<DirectedEdge> adjacentEdges = new Bag<>();

        for (int i = 0; i < adjacent.length; i++) {
            if (hasEdge(vertex, i)) {
                adjacentEdges.add(new DirectedEdge(vertex, i, adjacent[vertex][i]));
            }
        }

        return adjacentEdges;
    }

    public Iterable<DirectedEdge> edges() {
        Bag<DirectedEdge> edges = new Bag<>();

        for (int vertex = 0; vertex < vertices; vertex++) {
            for (DirectedEdge edge : adjacent(vertex)) {
                edges.add(edge);
            }
        }

        return edges;
    }

    @Override
    public String toString() {
        StringBuilder stringBuilder = new StringBuilder();

        for (int vertex = 0; vertex < vertices(); vertex++) {
            stringBuilder.append(vertex).append(": ");

            for (DirectedEdge neighbor : adjacent(vertex)) {
                stringBuilder.append(neighbor).append(" ");
            }
            stringBuilder.append("\n");
        }
        return stringBuilder.toString();
    }
}
