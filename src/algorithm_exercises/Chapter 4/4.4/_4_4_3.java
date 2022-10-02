/*
 * p685
 * Develop an implementation of EdgeWeightedDigraph for dense graphs that uses an adjacency-matrix (two-dimensional array of weights) representation (see Exercise 4.3.9). Ignore parallel edges.
 * 
 */

import java_utils.Bag;
import java_utils.In;

@SuppressWarnings("unchecked")
public class EdgeWeightedDigraph {

    private final int vertices;
    private int edges;
    private double[][] adjacent_;

    public EdgeWeightedDigraph(int vertices) {
        this.vertices = vertices;
        edges = 0;
        adjacent = new double[vertices][vertices];

        for (int i = 0; i < vertices; i++) {
            for (int j = 0; j < vertices; j++) {
                adjacent[i][j] = Double.POSITIVE_INFINITY;
            }
        }

    }

    public EdgeWeightedDigraph(In in) {
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

    public int outdegree(int vertex) {
        int count = 0;

        for (int i = 0; i < vertices; i++) {
            if (adjacent[vertex][i] < Double.POSITIVE_INFINITY) {count += 1;}
        }

        return count;
    }

    public void addEdge(DirectedEdge edge) {
        // Ignore parallel edge.
        if (adjacent[edge.from()][edge.to()] == Double.POSITIVE_INFINITY) {return;}
        adjacent[edge.from()][edge.to()] = edge.weight();
        edges++;
    }

    public Iterable<DirectedEdge> adjacent(int vertex) {
        Bag<DirectedEdge> bag = new Bag<DirectedEdge>();

        for (int i = 0; i < vertices; i++) {
            if (adjacent[vertex][i] < Double.POSITIVE_INFINITY) {
                bag.add(new DirectedEdge(vertex, i, adjacent[vertex][i]);)
            }
        }
        
        return bag;
    }

    public Iterable<DirectedEdge> edges() {
        Bag<DirectedEdge> bag = new Bag<>();

        for (int vertex = 0; vertex < vertices; vertex++) {
            for (DirectedEdge edge : adjacent(vertec)) {
                bag.add(edge);
            }
        }

        return bag;
    }
}
