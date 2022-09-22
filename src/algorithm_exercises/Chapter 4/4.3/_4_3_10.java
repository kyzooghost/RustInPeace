// p631

//  Develop an Edge Weighted Graph implementation for dense graphs that uses an adjacency-matrix (two-dimensional array of weights) representation. Disallow parallel edges.

import java_utils.In;

public class _4_3_10 {

    @SuppressWarnings("unchecked")
    public class EdgeWeightedGraph {

        private final int vertices;
        private int edges;
        private double[][] adjacent;

        public EdgeWeightedGraph(int vertices) {
            this.vertices = vertices;
            edges = 0;
            adjacent = new double[vertices][vertices];

            for (int i = 0; i < vertices; i++) {
                for (int j = 0; j < vertices; j++) {
                    adjacent[i][j] = -1.0;
                }
            }
        }

        public EdgeWeightedGraph(In in) {
            this(in.readInt());
            int edges = in.readInt();

            if (edges < 0) {
                throw new IllegalArgumentException("Number of edges must be nonnegative");
            }

            for (int i = 0; i < edges; i++) {
                int vertex1 = in.readInt();
                int vertex2 = in.readInt();
                double weight = in.readDouble();
                addEdge(vertex1, vertex2, weight);
            }
        }

        public int vertices() {
            return vertices;
        }

        public int edgesCount() {
            return edges;
        }

        public void addEdge(int vertex1_, int vertex2_, double weight_) {
            // Look for pre-existing edge, ignore if true (otherwise we add a parallel edge)
            if (adjacent[vertex1_][vertex2_] != -1.0) {
                return;
            }

            adjacent[vertex1_][vertex2_] = weight_;
            adjacent[vertex2_][vertex1_] = weight_;
            edges++;
        }

        public double[] adjacent(int vertex) {
            return adjacent[vertex];
        }

        public double[][] edges() {
            return adjacent;
        }

        @Override
        public String toString() {
            StringBuilder string = new StringBuilder();

            for (int i = 0; i < vertices(); i++) {
                for (int j = 0; j < vertices(); j++) {
                    if (i < j) {string.append(i).append(" - ").append(j).append(" : ").append(adjacent[i][j]);}
                }
            }

            return string.toString();
        }
    }
}
