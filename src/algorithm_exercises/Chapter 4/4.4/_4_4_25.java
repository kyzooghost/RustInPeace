/*
 * p688
 * Shortest path between two subsets. Given a digraph with positive edge weights, and two distinguished subsets of vertices S and T, find a shortest path from any vertex in S to any vertex in T. Your algorithm should run in time proportional to E log V, in the worst case.
 * 
 * This is basically multisource with a single twist, where our algorithm ran in time (E + V) log (V + 1) with the dummy vertex approach. If we instead initialize the priority queue with all sources, and set their distTo to 0, then we retain E log V time complexity.
 */

import java_utils.DirectedEdge;
import java_utils.DirectedEdgePath;
import java_utils.EdgeWeightedDigraph;
import java_utils.Stack;
import java_utils.IndexMinPriorityQueue;
import java_utils.HashSet;

public class _4_4_25 {

    public class DijkstraSPMultiSource {
        private DirectedEdge[] edgeTo;
        private double[] distTo;
        private IndexMinPriorityQueue<Double> pq;
        private double shortestDist;
        private int shortestDistTarget;

        public DijkstraSPMultiSource(EdgeWeightedDigraph G, HashSet<Integer> sources, HashSet<Integer> targets) {
            // Initialize data structures
            shortestDist = Double.POSITIVE_INFINITY;
            edgeTo = new DirectedEdge[G.vertices()];
            distTo = new double[G.vertices()];
            pq = new IndexMinPriorityQueue<Double>(G.vertices());
            for (int v = 0; v < G.vertices(); v++) {
                if (sources.contains(v)) {
                    distTo[v] = 0.0;
                } else {
                    distTo[v] = Double.POSITIVE_INFINITY;
                }
            }
            
            // Commence PQ operations
            for (int s : sources.keys()) {
                pq.insert(s, 0.0);
            }

            while (!pq.isEmpty()) {
                int vertexToRelax = pq.deleteMin();
                relax(G, vertexToRelax);
                if (targets.contains(vertexToRelax)) {
                    shortestDistTarget = vertexToRelax;
                    shortestDist = distTo[shortestDistTarget];
                    break;
                }
            }

            // More time-efficient to break early akin to source-sink shortest paths, rather than do an for-loop after the pq is empty.
            // Find shortest path
            // for (int t : targets.keys()) {
            //     if (distTo[t] < shortestDist) {
            //         shortestDist = distTo[t];
            //         shortestDistTarget = t;
            //     }
            // }
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

        private double distTo(int v) {
            return distTo[v];
        }

        private boolean hasPathTo(int v) {
            return edgeTo[v] != null;
        }

        private Iterable<DirectedEdge> pathTo(int v) {
            if (!hasPathTo(v)) return null;
            Stack<DirectedEdge> path = new Stack<DirectedEdge>();
            for (DirectedEdge e = edgeTo[v]; e != null; e = edgeTo[e.from()])
                path.push(e);
            return path;
        }

        public double shortestDist() {
            return shortestDist;
        }

        public int shortestDistTarget() {
            return shortestDistTarget;
        }

        public Iterable<DirectedEdge> shortestPath() {
            return pathTo(shortestDistTarget);
        }
    }

    public static void main(String[] args) {
        _4_4_25 exercise = new _4_4_25();
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

        HashSet<Integer> targets = new HashSet<>();
        targets.add(5);
        targets.add(6);

        DijkstraSPMultiSource sp = exercise.new DijkstraSPMultiSource(edgeWeightedDigraph, sources, targets);
        System.out.println(sp.shortestDist);
        System.out.println(sp.shortestDistTarget);
        for (DirectedEdge e: sp.shortestPath()) {
            System.out.println(e);
        }
    }

}
