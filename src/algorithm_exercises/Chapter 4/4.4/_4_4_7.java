/*
 * Develop a version of DijkstraSP that supports a client method that returns a second shortest path from s to t in an edge-weighted digraph (and returns null if there is only one shortest path from s to t).
 * 
 * How can you find the second shortest path
 * Ok I misinterpreted, I thought finding the second shortest path is getting the second MST. 
 * No we essentially need to find all paths from s to t, sort them all (or put them onto a priority queue), and find the second largest one.
 * Classic Dijkstra will only return one path, the path on the SPT.
 * 
 * Going back to non-weighted directed graphs. DFS will find some paths.
 * So we can use a modified DFS, that finds all paths from s to v
 * Classic DFS, can find some paths but not all, especially if some paths are different combinations of sub-paths
 * Classic DFS only visits each vertex and edge once, to find all paths we need to visit edges more than once, which means we're increasing time complexity beyond the original O(E + V)
 * 
 * So we can use an iterative DFS that maintains items on a stack, and deep copy it into an iterator whenever we find a path.
 * But how do we visit paths that aren't found through DFS (because they would involve re-visiting edges)
 * 
 * Approach from Github solutions - Starting from source, find every path (maintain a queue that you grow and fork).
 * Add each eligible path (with target at end) to a priority queue
 */

import java_utils.Stack;
import java_utils.PriorityQueue.Orientation;
import java_utils.PriorityQueue;
import java_utils.DirectedEdge;
import java_utils.EdgeWeightedDigraph;
import java.util.HashSet;
import java.util.ArrayList;
import java.util.Collections;

public class _4_4_7 {

    // Implementing Comparable<Path> => Can use in PriorityQueue and Collections.sort!
    public class Path implements Comparable<Path> {
        private int lastVertexInPath;
        private double weight;
        private HashSet<Integer> verticesInPath;
        private DirectedEdge lastEdgeInPath;
        // Reference to previous Path (Store as reference for O(1) constructor function, otherwise storing as collection and requiring deep clone creates an O(N) constructor function).
        private Path previousPath;

        // Create completely new path starting from single source
        Path(int source) {
            this.lastVertexInPath = source;
            this.weight = 0.0;
            this.verticesInPath = new HashSet<Integer>();
            this.verticesInPath.add(source);
            // lastEdgeInPath and previousPath initialised as `null`.
        }

        // Construct new Path object, by adding an edge to a previous Path.
        Path(Path previousPath, DirectedEdge directedEdge) {
            this.lastVertexInPath = directedEdge.to();
            this.previousPath = previousPath;
            this.lastEdgeInPath = directedEdge;
            this.weight = previousPath.weight() + directedEdge.weight();

            this.verticesInPath = new HashSet<Integer>();
            this.verticesInPath.addAll(previousPath.verticesInPath);
            this.verticesInPath.add(this.lastVertexInPath);
        }

        public double weight() {return this.weight;}
        public double directedEdge() {return this.directedEdge();}

        @Override
        public int compareTo(Path other) {
            if (this.weight < other.weight) {
                return -1;
            } else if (this.weight > other.weight) {
                return 1;
            } else {
                return 0;
            }
        }

        public Iterable<DirectedEdge> getPath() {
            Stack<DirectedEdge> path = new Stack<DirectedEdge>();
            path.push(this.lastEdgeInPath);

            Path iterator = previousPath;

            while (iterator != null && iterator.lastEdgeInPath != null) {
                path.push(iterator.lastEdgeInPath);
                iterator = iterator.previousPath;
            }

            return path;
        }
    }

    public class PathQuery {
        private ArrayList<Path> paths;

        public PathQuery(EdgeWeightedDigraph graph_, int source_, int target_) {
            // Ignoring validation check for source_ and target_

            this.paths = new ArrayList<Path>();
            PriorityQueue<Path> discoveredPaths = new PriorityQueue<Path>(Orientation.MIN);
            discoveredPaths.insert(new Path(source_));

            // O (Path length * number of paths * lg factor for priority-queue insertion) = O(V! * lg V!)
            // Because in worst case, from first vertex we have V paths, then from the 2nd vertex we have V-1 paths, and so on
            // So # of paths = V! / V
            // So this is a fairly straightforward and short implementation, made possible by the Path class, however the time complexity is gg.
            while (!discoveredPaths.isEmpty()) {
                Path currentPath = discoveredPaths.deleteTop();

                if (currentPath.lastVertexInPath == target_) {
                    this.paths.add(currentPath);
                }

                for (DirectedEdge e : graph_.adjacent(currentPath.lastVertexInPath)) {
                    if (!currentPath.verticesInPath.contains(e.to())) {
                        discoveredPaths.insert(new Path(currentPath, e));
                    }
                }
            }

            Collections.sort(this.paths);
        }

        public Path findKthSmallestPath(int k) {
            if (k > paths.size()) {return null;}
            return this.paths.get(k - 1);
        }
    }
    public static void main(String[] args) {
        _4_4_7 exercise = new _4_4_7();

        EdgeWeightedDigraph edgeWeightedDigraph = new EdgeWeightedDigraph(8);
        edgeWeightedDigraph.addEdge(new DirectedEdge(4, 5, 0.35));
        edgeWeightedDigraph.addEdge(new DirectedEdge(5, 4, 0.35));
        edgeWeightedDigraph.addEdge(new DirectedEdge(4, 7, 0.37));
        edgeWeightedDigraph.addEdge(new DirectedEdge(5, 7, 0.28));
        edgeWeightedDigraph.addEdge(new DirectedEdge(7, 5, 0.28));
        edgeWeightedDigraph.addEdge(new DirectedEdge(5, 1, 0.32));
        edgeWeightedDigraph.addEdge(new DirectedEdge(0, 4, 0.38));
        edgeWeightedDigraph.addEdge(new DirectedEdge(2, 0, 0.26));
        edgeWeightedDigraph.addEdge(new DirectedEdge(7, 3, 0.39));
        edgeWeightedDigraph.addEdge(new DirectedEdge(1, 3, 0.29));
        edgeWeightedDigraph.addEdge(new DirectedEdge(2, 7, 0.34));
        edgeWeightedDigraph.addEdge(new DirectedEdge(6, 2, 0.40));
        edgeWeightedDigraph.addEdge(new DirectedEdge(3, 6, 0.52));
        edgeWeightedDigraph.addEdge(new DirectedEdge(6, 0, 0.58));
        edgeWeightedDigraph.addEdge(new DirectedEdge(6, 4, 0.93));

        PathQuery pathQuery1 = exercise.new PathQuery(edgeWeightedDigraph, 2, 4);
        for (DirectedEdge e : pathQuery1.findKthSmallestPath(2).getPath()) {
            System.out.println(e.from() + "->" + e.to() + " " + e.weight());
        }

        /*
         * Expect:
         * 2->7 0.34
         * 7->5 0.28
         * 5->4 0.35
         */

        System.out.println("---");
        PathQuery pathQuery2 = exercise.new PathQuery(edgeWeightedDigraph, 6, 5);
        for (DirectedEdge e : pathQuery2.findKthSmallestPath(2).getPath()) {
            System.out.println(e.from() + "->" + e.to() + " " + e.weight());
        }

        /*
         * Expect:
         * 6->4 0.93
         * 4->5 0.35
         */

        System.out.println("---");
        PathQuery pathQuery3 = exercise.new PathQuery(edgeWeightedDigraph, 6, 2);
        if (pathQuery3.findKthSmallestPath(2) == null) {
            System.out.println("No second shortest path found");
        }
    }
}