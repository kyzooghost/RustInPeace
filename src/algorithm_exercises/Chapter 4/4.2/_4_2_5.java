// Modify Digraph to disallow parallel edges and self-loops.

import java_utils.Digraph;

public class _4_2_5 {
    public class DigraphPlus extends Digraph {
        public DigraphPlus(int vertices) {
            super(vertices);
        }

        public boolean hasEdge(int src, int tgt) {
            Iterable<Integer> outedges = this.adjacent(src);

            for (int outedge : outedges) {
                if (outedge == tgt) {return true;}
            }

            return false;
        }

        @Override
        public void addEdge(int vertex1, int vertex2) {
            if (vertex1 == vertex2 || this.hasEdge(vertex1, vertex2)) {return;} // No self-loop or parallel edge;

            this.adjacent[vertex1].add(vertex2);
            this.edges++;
    
            this.outdegrees[vertex1]++;
            this.indegrees[vertex2]++;
        }
    }
}
