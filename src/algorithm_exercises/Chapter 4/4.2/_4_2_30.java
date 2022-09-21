// Queue-based topological sort. 

// Develop a topological sort implementation that maintains a vertex-indexed array that keeps track of the indegree of each vertex. Initial- ize the array and a queue of sources in a single pass through all the edges, as in Exercise 4.2.7. Then, perform the following operations until the source queue is empty:
// ■ Remove a source from the queue and label it.
// ■ Decrement the entries in the indegree array corresponding to the destination vertex of each of the removed vertex’s edges.
// If decrementing any entry causes it to become 0, insert the corresponding vertex onto the source queue.

// Topological sort takes a Digraph
// Digraph stores the indegree of each vertex, which then means all the sources are stored
// We put all the sources on the queue

// Ahh got it, it's another way to do topological sort
// 1. Get collection of sources and put them on the queue
// 2. For each item on the queue, iterate through neighbors and decrement indegree by 1 (after removing source, what is the new source?)
// 3. If indegree == 0, queue onto source queue
// 4. Dequeue the queue to get topological order

// Sort of like a BFS

import java_utils.Digraph;
import java_utils.Queue;
import java.util.ArrayList;

public class _4_2_30 {
    public class TopologicalSort {
        private int[] sortOrder;

        public TopologicalSort (Digraph graph_) {
            Queue<Integer> sources = new Queue<Integer>();
            sortOrder = new int[(graph_.vertices())];
            int[] inDegree = new int[(graph_.vertices())];

            // 1. GET SOURCES
            for (int i = 0; i < graph_.vertices(); i++) {
                if (graph_.indegree(i) == 0) sources.enqueue(i);
                inDegree[i] = graph_.indegree(i);
            }

            // 2. ITERATE THROUGH QUEUE ITEMS
            int topologicalRank = 0;

            while (!sources.isEmpty()) {
                int vertex = sources.dequeue();

                // 3. ITERATE THROUGH NEIGHBORS
                for (int i : graph_.adjacent(vertex)) {
                    inDegree[i] -= 1;
                    if (inDegree[i] == 0) sources.enqueue(i);
                }

                sortOrder[topologicalRank] = vertex;
                topologicalRank += 1;
            }
        }
    }
}
