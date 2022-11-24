/*
 * p689
 * Monotonic shortest path. Given a weighted digraph, find a monotonic shortest path from s to every other vertex. A path is monotonic if the weight of every edge on the path is either strictly increasing or strictly decreasing. The path should be simple (no repeated vertices). Hint : Relax edges in ascending order and find a best path; then relax edges in descending order and find a best path.
 * 
 * Monotonic - weight of every edge is strictly increasing or decreasing.
 * 
 * ALGORITHM
 * 
 * For each vertex, store sorted array of outgoing edges
 * 
 * Use a minimum priority queue of paths
 *      Initialize priority queue with all one-edge paths from the source
 * 
 * While priority queue is not empty
 *      Consider all outgoing edges from end of path, add to queue if find an edge that maintain monotonic increasing/decreasing quality
 *      Modify edgeTo[] and distTo[] entries if you find an eligible path extension, or if non-null
 * 
 * QUESTIONS
 * 
 * What about overwrites or edgeTo[] and distTo[]?
 *    No overwrites unless the edge is added to the queue (which means it must extend the monotonically increasing or decreasing path).
 * 
 * And why does this guarantee that each edge is processed at most once?
 *    Ahh I get it, everytime you visit an edge you mark it as visited, so you can't revisit it.
 *    So yes, O(E lg E) is the right time complexity
 * 
 * So it all depends on whether you sort the outgoing edges by ascending or descending order.
 * 
 * If you sort by descending order, and enforce monotonically increasing path
 *      If you process outedges in descending order, you can save redundant processing
 *      Because if you process the biggest edge, and it doesn't extend the monotonically increasing path, then it is redundant to process any smaller edge.
 * 
 * If you sort by ascending order, and enforce monotonically decreasing path
 * 
 * Longest path overwrites the edgeTo[] and distTo[] entries => Gets priority
 * 
 * How is this a modified Dijkstra's?
 *    You are starting from the source
 *    And you visit that is one edge removed from the current SPT
 */


import java_utils.Stack;
import java_utils.SeparateChainingHashTable;
import java_utils.DirectedEdge;
import java_utils.EdgeWeightedDigraph;
import java.util.*;

/**
 * Created by Rene Argento on 10/12/17.
 */
// Based on https://stackoverflow.com/questions/22876105/find-a-monotonic-shortest-path-in-a-graph-in-oe-logv
public class _4_4_34 {

    public class Path {
        private double weight;
        private DirectedEdge lastEdge;

        Path(double weight, DirectedEdge lastEdge) {
            this.weight = weight;
            this.lastEdge = lastEdge;
        }

        public double weight() {
            return weight;
        }

        public DirectedEdge lastEdge() {
            return lastEdge;
        }
    }

    // For each vertex, maintain array of outgoing edges
    public class VertexInformation {

        private DirectedEdge[] edges;
        private int currentEdgeIteratorPosition;

        VertexInformation(DirectedEdge[] edges) {
            this.edges = edges;
            this.currentEdgeIteratorPosition = 0;
        }

        public void incrementEdgeIteratorPosition() {
            currentEdgeIteratorPosition++;
        }

        public DirectedEdge[] getEdges() {
            return edges;
        }

        public int getCurrentEdgeIteratorPosition() {
            return currentEdgeIteratorPosition;
        }
    }

    public class DijkstraMonotonicSP {

        private double[] distTo;                            // length of path to vertex
        private DirectedEdge[] edgeTo;                      // last edge on path to vertex

        private double[] distToMonotonicAscending;          // length of monotonic ascending path to vertex
        private DirectedEdge[] edgeToMonotonicAscending;    // last edge on monotonic ascending path to vertex

        private double[] distToMonotonicDescending;         // length of monotonic descending path to vertex
        private DirectedEdge[] edgeToMonotonicDescending;   // last edge on monotonic descending path to vertex

        // O(E lg E)
        // If negative edge weights are present, still works but become O(2^V)
        public DijkstraMonotonicSP(EdgeWeightedDigraph edgeWeightedDigraph, int source) {
            distToMonotonicAscending = new double[edgeWeightedDigraph.vertices()];
            distToMonotonicDescending = new double[edgeWeightedDigraph.vertices()];
            distTo = new double[edgeWeightedDigraph.vertices()];

            edgeToMonotonicAscending = new DirectedEdge[edgeWeightedDigraph.vertices()];
            edgeToMonotonicDescending = new DirectedEdge[edgeWeightedDigraph.vertices()];
            edgeTo = new DirectedEdge[edgeWeightedDigraph.vertices()];

            for (int vertex = 0; vertex < edgeWeightedDigraph.vertices(); vertex++) {
                distTo[vertex] = Double.POSITIVE_INFINITY;
                distToMonotonicAscending[vertex] = Double.POSITIVE_INFINITY;
                distToMonotonicDescending[vertex] = Double.POSITIVE_INFINITY;
            }

            // 1- Relax edges in ascending order to get a monotonic increasing shortest path
            Comparator<DirectedEdge> edgesComparator = new Comparator<DirectedEdge>() {
                @Override
                public int compare(DirectedEdge edge1, DirectedEdge edge2) {
                    if (edge1.weight() > edge2.weight()) {
                        return -1;
                    } else if (edge1.weight() < edge2.weight()) {
                        return 1;
                    } else {
                        return 0;
                    }
                }
            };

            relaxAllEdgesInSpecificOrder(edgeWeightedDigraph, source, edgesComparator, distToMonotonicAscending,
                    edgeToMonotonicAscending, true);

            System.out.println("------");

            // 2- Relax edges in descending order to get a monotonic decreasing shortest path
            edgesComparator = new Comparator<DirectedEdge>() {
                @Override
                public int compare(DirectedEdge edge1, DirectedEdge edge2) {
                    if (edge1.weight() < edge2.weight()) {
                        return -1;
                    } else if (edge1.weight() > edge2.weight()) {
                        return 1;
                    } else {
                        return 0;
                    }
                }
            };

            relaxAllEdgesInSpecificOrder(edgeWeightedDigraph, source, edgesComparator, distToMonotonicDescending,
                    edgeToMonotonicDescending, false);

            // 3- Compare distances to get the shortest monotonic path
            compareMonotonicPathsAndComputeShortest();
        }

        private void relaxAllEdgesInSpecificOrder(EdgeWeightedDigraph edgeWeightedDigraph, int source,
                                                  Comparator<DirectedEdge> edgesComparator, double[] distToVertex,
                                                  DirectedEdge[] edgeToVertex, boolean isAscendingOrder) {
            // Create a map with vertices as keys and sorted outgoing edges as values
            // Vertex => VertexInformation (outgoing edges)
            SeparateChainingHashTable<Integer, VertexInformation> verticesInformation = new SeparateChainingHashTable<>();

            // For each vertex, maintained sorted list of edges (either ascending or descending by weight)
            for (int vertex = 0; vertex < edgeWeightedDigraph.vertices(); vertex++) {
                DirectedEdge[] edges = new DirectedEdge[edgeWeightedDigraph.outdegree(vertex)];

                int edgeIndex = 0;
                for (DirectedEdge edge : edgeWeightedDigraph.adjacent(vertex)) {
                    edges[edgeIndex++] = edge;
                }

                Arrays.sort(edges, edgesComparator);

                verticesInformation.put(vertex, new VertexInformation(edges));
            }

            // Hmm minimum priority queue of paths
            PriorityQueue<Path> priorityQueue = new PriorityQueue<>(new Comparator<Path>() {
                @Override
                public int compare(Path path1, Path path2) {
                    if (path1.weight() < path2.weight()) {
                        return -1;
                    } else if (path1.weight() > path2.weight()) {
                        return 1;
                    } else {
                        return 0;
                    }
                }
            });

            distToVertex[source] = 0;

            VertexInformation sourceVertexInformation = verticesInformation.get(source);
            
            // For each outgoing edge from the source, create a path starting with that edge and add it to the priority queue.
            while (sourceVertexInformation.currentEdgeIteratorPosition < sourceVertexInformation.getEdges().length) {
                DirectedEdge edge = sourceVertexInformation.getEdges()[sourceVertexInformation.getCurrentEdgeIteratorPosition()];
                sourceVertexInformation.incrementEdgeIteratorPosition();
                Path path = new Path(edge.weight(), edge);
                priorityQueue.offer(path);
            }

            while (!priorityQueue.isEmpty()) {
                // Take the vertex at the tip of the path, get its outgoing edges
                Path currentShortestPath = priorityQueue.poll();
                DirectedEdge currentEdge = currentShortestPath.lastEdge();
                int nextVertexInPath = currentEdge.to();
                VertexInformation nextVertexInformation = verticesInformation.get(nextVertexInPath);
                double weightInPreviousEdge = currentEdge.weight();

                System.out.println(currentEdge);

                // Iterate through each outgoing edge
                while (nextVertexInformation.getCurrentEdgeIteratorPosition() < nextVertexInformation.getEdges().length) {
                    DirectedEdge edge =
                            verticesInformation.get(nextVertexInPath).getEdges()[nextVertexInformation.getCurrentEdgeIteratorPosition()];
                    System.out.println("   consider " + edge);
                    // Break if ascending or descending trend broken
                    if ((isAscendingOrder && edge.weight() <= weightInPreviousEdge)
                            || (!isAscendingOrder && edge.weight() >= weightInPreviousEdge)) {
                        break;
                    }

                    nextVertexInformation.incrementEdgeIteratorPosition();

                    // We have discovered path can continue from vertex, so store path in 
                    edgeToVertex[nextVertexInPath] = currentShortestPath.lastEdge();
                    distToVertex[nextVertexInPath] = currentShortestPath.weight();

                    // Extend path and add to priority queue
                    Path path = new Path(currentShortestPath.weight() + edge.weight(), edge);
                    priorityQueue.offer(path);
                    System.out.println("   added to queue: " + edge);
                }
                
                // If we didn't find any eligible outgoing edge and no existing edge (so no overwrites, fill in edgeTo[] and distTo[])
                if (edgeToVertex[nextVertexInPath] == null) {
                    edgeToVertex[nextVertexInPath] = currentEdge;
                    distToVertex[nextVertexInPath] = currentShortestPath.weight();
                }

                System.out.println("  currentEdge: " + currentEdge);

                for (int vertex = 0; vertex < edgeWeightedDigraph.vertices(); vertex++) {
                    System.out.println("      edgeTo[" + vertex + "]: " + edgeToVertex[vertex]);
                }
            }
        }

        private void compareMonotonicPathsAndComputeShortest() {
            for (int vertex = 0; vertex < edgeTo.length; vertex++) {
                if (distToMonotonicAscending[vertex] <= distToMonotonicDescending[vertex]) {
                    distTo[vertex] = distToMonotonicAscending[vertex];
                    edgeTo[vertex] = edgeToMonotonicAscending[vertex];
                } else {
                    distTo[vertex] = distToMonotonicDescending[vertex];
                    edgeTo[vertex] = edgeToMonotonicDescending[vertex];
                }
            }
        }

        public double distTo(int vertex) {
            return distTo[vertex];
        }

        public boolean hasPathTo(int vertex) {
            return distTo[vertex] != Double.POSITIVE_INFINITY;
        }

        public Iterable<DirectedEdge> pathTo(int vertex) {
            if (!hasPathTo(vertex)) {
                return null;
            }

            Stack<DirectedEdge> path = new Stack<>();
            for (DirectedEdge edge = edgeTo[vertex]; edge != null; edge = edgeTo[edge.from()]) {
                path.push(edge);
            }
            return path;
        }
    }

    public static void main(String[] args) {
        _4_4_34 exercise = new _4_4_34();

        EdgeWeightedDigraph edgeWeightedDigraph1 = new EdgeWeightedDigraph(8);
        edgeWeightedDigraph1.addEdge(new DirectedEdge(0, 1, 1));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(0, 2, 4));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(0, 3, 9));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(1, 2, 2));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(1, 3, 6));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(2, 3, 3));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(3, 4, 4));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(3, 6, 8));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(4, 5, 5));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(5, 3, 7));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(5, 6, 6));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(6, 7, 7));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(6, 2, 5));
        edgeWeightedDigraph1.addEdge(new DirectedEdge(7, 5, 9));

        DijkstraMonotonicSP dijkstraMonotonicSP1 = exercise.new DijkstraMonotonicSP(edgeWeightedDigraph1, 0);

        // StdOut.print("Monotonic shortest paths 1: ");

        // for (int vertex = 0; vertex < edgeWeightedDigraph1.vertices(); vertex++) {
        //     StdOut.print("\nPath from vertex 0 to vertex " + vertex + ": ");

        //     if (dijkstraMonotonicSP1.hasPathTo(vertex)) {
        //         for (DirectedEdge edge : dijkstraMonotonicSP1.pathTo(vertex)) {
        //             StdOut.print(edge.from() + "->" + edge.to() + " (" + edge.weight() + ") ");
        //         }
        //     } else {
        //         StdOut.print("There is no monotonic path to vertex " + vertex);
        //     }
        // }

        // StdOut.println("\n\nExpected monotonic paths");
        // StdOut.println("Vertex 0: ");
        // StdOut.println("Vertex 1: 0->1 (1.0)");
        // StdOut.println("Vertex 2: 0->1 (1.0) 1->2 (2.0)");
        // StdOut.println("Vertex 3: 0->1 (1.0) 1->3 (0.0)");
        // StdOut.println("Vertex 4: 0->4 (3.0)");
        // StdOut.println("Vertex 5: 0->1 (1.0) 1->5 (0.0)");
        // StdOut.println("Vertex 6: There is no monotonic path to vertex 6"); // There is a path but it is not monotonic
        // StdOut.println("Vertex 7: There is no monotonic path to vertex 7"); // There is a path but it is not monotonic


        // EdgeWeightedDigraph edgeWeightedDigraph2 = new EdgeWeightedDigraph(3);
        // edgeWeightedDigraph2.addEdge(new DirectedEdge(0, 1, 1));
        // edgeWeightedDigraph2.addEdge(new DirectedEdge(0, 1, 4));
        // edgeWeightedDigraph2.addEdge(new DirectedEdge(1, 2, 3));

        // DijkstraMonotonicSP dijkstraMonotonicSP2 =
        //         new Exercise34_MonotonicShortestPath().new DijkstraMonotonicSP(edgeWeightedDigraph2, 0);

        // StdOut.print("\nMonotonic shortest paths 2: ");

        // for (int vertex = 0; vertex < edgeWeightedDigraph2.vertices(); vertex++) {
        //     StdOut.print("\nPath from vertex 0 to vertex " + vertex + ": ");

        //     if (dijkstraMonotonicSP2.hasPathTo(vertex)) {
        //         for (DirectedEdge edge : dijkstraMonotonicSP2.pathTo(vertex)) {
        //             StdOut.print(edge.from() + "->" + edge.to() + " (" + edge.weight() + ") ");
        //         }
        //     } else {
        //         StdOut.print("There is no monotonic path to vertex " + vertex);
        //     }
        // }

        // StdOut.println("\n\nExpected monotonic paths");
        // StdOut.println("Vertex 0: ");
        // StdOut.println("Vertex 1: 0->1 (1.0)");
        // StdOut.println("Vertex 2: 0->1 (1.0) 1->2 (3.0)");
    }
}