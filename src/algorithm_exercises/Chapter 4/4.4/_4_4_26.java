/*
 * p688
 * Single-source shortest paths in dense graphs. Develop a version of Dijkstra’s algorithm that can find the SPT from a given vertex in a dense edge-weighted digraph in time proportional to V^2. Use an adjacency-matrix representation (see Exercise 4.4.3 and Exercise 4.3.29).
 * 
 * V^2 vs E lg V, in a dense graph E ~= V^2, so your time efficiency ~= O(V^2 lg V) with a pq-based Dijstra's in a dense graph
 * Dijkstra's algorithm = relax vertex in ascending order of distTo[], we used a minPQ for this previously. Here we will pay an O(N) cost to find the minimum.
 */

package exercise_4_4;   

import java_utils.DirectedEdge;
import java_utils.DirectedEdgePath;
import java_utils.Stack;
import java_utils.IndexMinPriorityQueue;
import java_utils.HashSet;
import java_utils.EdgeWeightedDigraphAdjacencyMatrix;

public class _4_4_26 {

    public class DijkstraSPDense {
        private DirectedEdge[] edgeTo;
        private double[] distTo;
        private IndexMinPriorityQueue<Double> pq;

        public DijkstraSPDense(EdgeWeightedDigraphAdjacencyMatrix G, int source) {
            // Initialize data structures
            edgeTo = new DirectedEdge[G.vertices()];
            distTo = new double[G.vertices()];
            pq = new IndexMinPriorityQueue<Double>(G.vertices());
            for (int v = 0; v < G.vertices(); v++) {
                distTo[v] = Double.POSITIVE_INFINITY;
            }
            distTo[source] = 0.0;            

            boolean[] relaxed = new boolean[G.vertices()];
            int nextVertexToRelax = source;

            // We want to relax edge vertex, in order of lowest distTo[] to highest
            for (int i = 0; i < G.vertices(); i++) {
                nextVertexToRelax = relax(G, nextVertexToRelax, relaxed);
            }
        }

        private int relax(EdgeWeightedDigraphAdjacencyMatrix G, int v, boolean[] relaxed) {
            relaxed[v] = true;

            // Relax all outedges
            for (DirectedEdge e : G.adjacent(v)) {
                int w = e.to();
                if (distTo[w] > distTo[v] + e.weight()) {
                    distTo[w] = distTo[v] + e.weight();
                    edgeTo[w] = e;
                }
            } 

            // Find next vertex to relax - smallest distTo[] value, that we haven't relaxed already
            int nextVertexToRelax = -1;
            double minimumWeight = Double.POSITIVE_INFINITY;
            for (int i = 0; i < G.vertices(); i++) {
                if (!relaxed[i] && distTo[i] < minimumWeight) {
                    nextVertexToRelax = i;
                    minimumWeight = distTo[i];
                }
            }
            return nextVertexToRelax;
        }

        public double distTo(int v) {
            return distTo[v];
        }

        public boolean hasPathTo(int v) {
            return edgeTo[v] != null;
        }

        public Iterable<DirectedEdge> pathTo(int v) {
            if (!hasPathTo(v)) return null;
            Stack<DirectedEdge> path = new Stack<DirectedEdge>();
            for (DirectedEdge e = edgeTo[v]; e != null; e = edgeTo[e.from()])
                path.push(e);
            return path;
        }
    }

    public static void main(String[] args) {
        _4_4_26 exercise = new _4_4_26();
        // tinyEWD.txt or digraph on p644
        EdgeWeightedDigraphAdjacencyMatrix edgeWeightedDigraph = new EdgeWeightedDigraphAdjacencyMatrix(8);
        edgeWeightedDigraph.addEdge(new DirectedEdge(4, 5, 0.35));
        edgeWeightedDigraph.addEdge(new DirectedEdge(5, 4, 0.35));
        edgeWeightedDigraph.addEdge(new DirectedEdge(4, 7, 0.37));
        edgeWeightedDigraph.addEdge(new DirectedEdge(5, 7, 0.28));
        edgeWeightedDigraph.addEdge(new DirectedEdge(7, 5, 0.28));
        edgeWeightedDigraph.addEdge(new DirectedEdge(5, 1, 0.32));
        edgeWeightedDigraph.addEdge(new DirectedEdge(0, 4, 0.38));
        edgeWeightedDigraph.addEdge(new DirectedEdge(0, 2, 0.26));
        edgeWeightedDigraph.addEdge(new DirectedEdge(7, 3, 0.39));
        edgeWeightedDigraph.addEdge(new DirectedEdge(1, 3, 0.29));
        edgeWeightedDigraph.addEdge(new DirectedEdge(2, 7, 0.34));
        edgeWeightedDigraph.addEdge(new DirectedEdge(6, 2, 0.40));
        edgeWeightedDigraph.addEdge(new DirectedEdge(3, 6, 0.52));
        edgeWeightedDigraph.addEdge(new DirectedEdge(6, 0, 0.58));
        edgeWeightedDigraph.addEdge(new DirectedEdge(6, 4, 0.93));

        // p653 for answers
        DijkstraSPDense sp = exercise.new DijkstraSPDense(edgeWeightedDigraph, 0);
        System.out.println(sp.distTo(0));
        System.out.println(sp.distTo(1));
        System.out.println(sp.distTo(2));
        System.out.println(sp.distTo(3));
        System.out.println(sp.distTo(4));
        System.out.println(sp.distTo(5));
        System.out.println(sp.distTo(6));
        System.out.println(sp.distTo(7));

        for (DirectedEdge e : sp.pathTo(3)) {
            System.out.println(e);
        }
    }

}
