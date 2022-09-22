// 632
// Given an MST for an edge-weighted graph G and a new edge e, write a program that determines the range of weights for which e is in an MST.

import java_utils.EdgeWeightedGraph;
import java_utils.Edge;
import java_utils.Queue;

public class _4_3_16 {
    public class EdgeWeightedGraphPlus {
        private Queue<Edge> mst;

        public EdgeWeightedGraphPlus (Queue<Edge> mst) {
            this.mst = mst;
        }

        public double getMaxWeightForNewMST(Edge edge_) {
            double maxWeight = findMaxWeight(edge_);
            return maxWeight;
        }

        private double findMaxWeight(Edge edge_) {
            int vertex1 = edge_.either();
            int vertex2 = edge_.other(vertex1);
            Queue<Integer> bfsQueue = new Queue<Integer>();
            Edge[] edgeTo = new Edge[mst.size() + 1];
            boolean[] visited = new boolean[mst.size() + 1];
            edgeTo[vertex1] = null; // If we are doing BFS from vertex1, and there is no cycle, we cannot have an edge to vertex1

            // Create graph with MST
            EdgeWeightedGraph graph = new EdgeWeightedGraph(mst.size() + 1);
            for (Edge e : mst) {graph.addEdge(e);}

            // BFS starting from vertex1, to fill edgeTo[] array (ceebs creating consumable iterator for iterative DFS)
            bfsQueue.enqueue(vertex1);

            while (!bfsQueue.isEmpty()) {
                int vertex = bfsQueue.dequeue();
                visited[vertex] = true;

                for (Edge e: graph.adjacent(vertex)) {
                    // Skip if visited (we store duplicates of each edge in EdgeWeightedGraph implementation)
                    if (visited[e.other(vertex)]) {continue;}
                    int otherVertex = e.other(vertex);
                    edgeTo[otherVertex] = e;
                    bfsQueue.enqueue(otherVertex);
                }
            }

            // Iterate backwards from vertex2 in edgeTo[] to find the max-weighted edge in path from vertex1 to vertex2
            double maxWeight = 0;
            int pointerVertex = vertex2;

            while (pointerVertex != vertex1) {
                Edge e = edgeTo[pointerVertex];
                if (e.weight() > maxWeight) maxWeight = e.weight();
                pointerVertex = e.other(pointerVertex);
            }

            return maxWeight;
        }
    }

    public static void main(String[] args) {
        _4_3_16 exercise = new _4_3_16();

        Queue<Edge> mst = new Queue<Edge>();
        mst.enqueue(new Edge(0, 7, 0.16));
        mst.enqueue(new Edge(2, 3, 0.17));
        mst.enqueue(new Edge(1, 7, 0.19));
        mst.enqueue(new Edge(0, 2, 0.26));
        mst.enqueue(new Edge(5, 7, 0.28));
        mst.enqueue(new Edge(4, 5, 0.35));
        mst.enqueue(new Edge(6, 2, 0.40));

        EdgeWeightedGraphPlus query = exercise.new EdgeWeightedGraphPlus(mst);
        System.out.println(query.getMaxWeightForNewMST(new Edge(1, 3, 0.01)));
        System.out.println(query.getMaxWeightForNewMST(new Edge(0, 4, 0.01)));
    }
}
