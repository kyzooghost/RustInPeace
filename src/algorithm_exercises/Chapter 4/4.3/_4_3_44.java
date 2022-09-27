/*
 * Improved Boruvka. 
 * 
 * Develop an implementation of Boruvka’s algorithm that uses doubly-linked circular lists to represent MST subtrees so that subtrees can be merged and renamed in time proportional to E during each stage (and the union-find ADT is therefore not needed).
 * 
 * Hmm, each MST subtree => doubly-linked circular list, that can be merged into one another
 * How to store a collection of doubly-linked circular lists? As an ArrayList
 */

import java_utils.DoublyLinkedListCircular;
import java_utils.EdgeWeightedGraph;
import java_utils.Edge;
import java_utils.Queue;
import java_utils.HashSet;

public class _4_3_44 {
    public class ImprovedBoruvkaMST {
        private Queue<Edge> mst;
        private double weight;

        public ImprovedBoruvkaMST(EdgeWeightedGraph graph_) {
            mst = new Queue<Edge>();

            // Create array of subtrees, subtrees[vertex] = Subtree at vertex
            DoublyLinkedListCircular<Integer>[] subtrees = new DoublyLinkedListCircular[graph_.vertices()];
            for (int i = 0; i < graph_.vertices(); i++) {
                subtrees[i] = new DoublyLinkedListCircular<Integer>();
                subtrees[i].insertAtTheBeginning(i);
            }

            for (int stage = 0; stage < graph_.vertices(); stage = stage + stage) {
                if (mst.size() == graph_.vertices() - 1) {break;}
                Edge[] closestEdges = new Edge[graph_.vertices()];

                // Find smallest weight edge for each vertex, that is not in the same subtree.
                for (Edge edge : graph_.edges()) {
                    int vertex1 = edge.either();
                    int vertex2 = edge.other(vertex1);
                    int subtreeID1 = subtrees[vertex1].first().item;
                    int subtreeID2 = subtrees[vertex2].first().item;

                    // Skip if in same subtree.
                    if (subtreeID1 == subtreeID2) {continue;}

                    // Update to store smallest weight edge.
                    if (closestEdges[subtreeID1] == null || edge.weight() < closestEdges[subtreeID1].weight()) {
                        closestEdges[subtreeID1] = edge;
                    }
                    if (closestEdges[subtreeID2] == null || edge.weight() < closestEdges[subtreeID2].weight()) {
                        closestEdges[subtreeID2] = edge;
                    }
                }

                for (int vertex = 0; vertex < graph_.vertices(); vertex++) {
                    Edge closestEdge = closestEdges[vertex];

                    if (closestEdge != null) {
                        int vertex1 = closestEdge.either();
                        int vertex2 = closestEdge.other(vertex1);
                        int subtreeID1 = subtrees[vertex1].first().item;
                        int subtreeID2 = subtrees[vertex2].first().item;
                        if (subtreeID1 == subtreeID2) {continue;}

                        // Add to MST
                        mst.enqueue(closestEdge);
                        weight += closestEdge.weight();

                        // Merge subtrees
                        HashSet<Integer> verticesToBeUpdated = new HashSet<Integer>();
                        for (int i : subtrees[vertex2]) {verticesToBeUpdated.add(i);}
                        subtrees[vertex1].insertLinkedListAtTheEnd(subtrees[vertex2]);
                        // Update references for subtrees[] indexes in merged subtree => This is key step enabling succinct code.
                        for (int i : verticesToBeUpdated.keys()) {subtrees[i] = subtrees[vertex1];}
                    }
                }
            }
        }

        public Iterable<Edge> edges() {
            return mst;
        }

        public double weight() {
            return weight;
        }
    }

    public static void main(String[] args) {
        _4_3_44 exercise = new _4_3_44();
        EdgeWeightedGraph edgeWeightedGraph = new EdgeWeightedGraph(5);
        edgeWeightedGraph.addEdge(new Edge(0, 1, 0.42));
        edgeWeightedGraph.addEdge(new Edge(0, 3, 0.5));
        edgeWeightedGraph.addEdge(new Edge(1, 2, 0.12));
        edgeWeightedGraph.addEdge(new Edge(1, 4, 0.91));
        edgeWeightedGraph.addEdge(new Edge(2, 3, 0.72));
        edgeWeightedGraph.addEdge(new Edge(3, 4, 0.8));
        edgeWeightedGraph.addEdge(new Edge(3, 4, 0.82));
        edgeWeightedGraph.addEdge(new Edge(4, 4, 0.1));

        ImprovedBoruvkaMST mst = exercise.new ImprovedBoruvkaMST(edgeWeightedGraph);
        for (Edge edge : mst.edges()) {System.out.println(edge);}
    }
}
