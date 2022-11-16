/*
 * p688
 * Source-sink shortest paths. Develop an API and implementation that use a version of Dijkstra’s algorithm to solve the source-sink shortest path problem on edge-weighted digraphs.
 * 
 * Source-sink shortest path - given an edge-weighted digraph, a source vertex s,and a target vertex t, find the shortest path from s to t.
 * 
 * Initialize dist[s] to 0, and all other distTo[] entries to positive infinity
 * Relax and add to the SPT a non-tree vertex with lowest distTo[] value, until all vertices on the tree or no non-tree vertex has a finite distTo[] value.
 * Add next non-tree vertex closest to the source.
 * Source-sink - Dijkstra's but terminate search as soon as t comes off the priority queue.
 * O(E lg V) time complexity? We use a priority queue, where each operation is lg V complexity, for each edge.
 */

import java_utils.DirectedEdge;
import java_utils.DirectedEdgePath;
import java_utils.EdgeWeightedDigraph;
import java_utils.Stack;
import java_utils.IndexMinPriorityQueue;

public class _4_4_23 {

    public class DijkstraSPModified {
        private DirectedEdge[] edgeTo;
        private double[] distTo;
        private IndexMinPriorityQueue<Double> pq;

        public DijkstraSPModified(EdgeWeightedDigraph G, int s, int t) {
            // Initialize data structures
            edgeTo = new DirectedEdge[G.vertices()];
            distTo = new double[G.vertices()];
            pq = new IndexMinPriorityQueue<Double>(G.vertices());
            for (int v = 0; v < G.vertices(); v++)
                distTo[v] = Double.POSITIVE_INFINITY;
            distTo[s] = 0.0;
            pq.insert(s, 0.0);

            while (!pq.isEmpty()) {
                System.out.println(pq.minIndex());
                if (pq.minIndex() == t) break;
                relax(G, pq.deleteMin());
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
        _4_4_23 exercise = new _4_4_23();
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

        DijkstraSPModified sp = exercise.new DijkstraSPModified(edgeWeightedDigraph, 0, 2);
        System.out.println(sp.distTo(2));
        DijkstraSPModified sp1 = exercise.new DijkstraSPModified(edgeWeightedDigraph, 0, 5);
        System.out.println(sp1.distTo(5));
        System.out.println(sp1.distTo(6));
    }

}
