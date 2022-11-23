/*
 * p688
 * Multisource shortest paths. Develop an API and implementation that uses Dijkstra’s algorithm to solve the multisource shortest-paths problem on edge-weighted digraphs with positive edge weights: given a set of sources, find a shortest-paths forest that enables implementation of a method that returns to clients the shortest path from any source to each vertex. Hint : Add a dummy vertex with a zero-weight edge to each source, or initialize the priority queue with all sources, with their distTo[] entries set to 0.
 * 
 * From any source, to any vertex.
 * Initialize PQ with all sources (instead of the one source), with all distTo[] entries set to 0.
 * If all distTo[] entries are set to 0, then relax does anything
 * Ahh it is given a set of sources, not assuming that every vertex is now a source (in which case the distTo is automatically 0.)
 */

import java_utils.DirectedEdge;
import java_utils.DirectedEdgePath;
import java_utils.EdgeWeightedDigraph;
import java_utils.Stack;
import java_utils.IndexMinPriorityQueue;
import java_utils.HashSet;

public class _4_4_24 {

    public class DijkstraSPMultiSource {
        private DirectedEdge[] edgeTo;
        private double[] distTo;
        private IndexMinPriorityQueue<Double> pq;

        public DijkstraSPMultiSource(EdgeWeightedDigraph G, HashSet<Integer> sources) {
            EdgeWeightedDigraph graphWithDummySource = new EdgeWeightedDigraph(G.vertices() + 1);
            int dummyVertex = G.vertices();

            // Initialize data structures
            edgeTo = new DirectedEdge[G.vertices() + 1];
            distTo = new double[G.vertices() + 1];
            pq = new IndexMinPriorityQueue<Double>(G.vertices() + 1);
            for (int v = 0; v < G.vertices(); v++) {
                distTo[v] = Double.POSITIVE_INFINITY;
            }
            distTo[dummyVertex] = 0.0;
            
            // Copy edges
            for (DirectedEdge e : G.edges()) {
                graphWithDummySource.addEdge(e);
            }

            // Add zero-weight edges from dummySource to sources
            for (int s : sources.keys()) {
                graphWithDummySource.addEdge(new DirectedEdge(dummyVertex, s, 0.0));
            }

            // Commence PQ operations
            pq.insert(dummyVertex, 0.0);
            while (!pq.isEmpty()) {
                relax(graphWithDummySource, pq.deleteMin());
            }
        }

        private void relax(EdgeWeightedDigraph G, int v) {
            for (DirectedEdge e : G.adjacent(v)) {
                int w = e.to();
                if (distTo[w] > distTo[v] + e.weight()) {
                    distTo[w] = distTo[v] + e.weight();
                    edgeTo[w] = e;
                    if (pq.contains(w)) pq.changeKey(w, distTo[w]);
                    else                pq.insert(w, distTo[w]);
                }
            } 
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
        _4_4_24 exercise = new _4_4_24();
        // tinyEWD.txt or digraph on p644
        EdgeWeightedDigraph edgeWeightedDigraph = new EdgeWeightedDigraph(8);
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

        HashSet<Integer> sources = new HashSet<>();
        sources.add(0);
        sources.add(1);
        sources.add(7);

        DijkstraSPMultiSource sp = exercise.new DijkstraSPMultiSource(edgeWeightedDigraph, sources);
        System.out.println(sp.distTo(5));
        System.out.println(sp.distTo(6));
    }

}
