// Indegree of a vertex => How many directed edges point to that vertex
// Outdegree of a vertex => How many directed edges emanate from that vertex
// Sink => Vertex with outdegree 0, no vertex reachable from a sink
// Source => Vertex with indegree 0, not reachable from any other vertex
// Map => Digraph where self-loop allowed and every vertex has outdegree 1

import java_utils.Digraph;
import java.util.ArrayList;

public class _4_2_7 {
    public class Degree {
        private Digraph graph;

        public Degree (Digraph g) {
            this.graph = g;
        }

        public int indegree (int v) {
            Digraph reverse = graph.reverse();
            Iterable<Integer> vertices = reverse.adjacent(v);
            int count = 0;
            for (int i : vertices) {
                count ++;
            }
            return count;
        }

        public int outdegree (int v) {
            Iterable<Integer> vertices = graph.adjacent(v);
            int count = 0;
            for (int i : vertices) {
                count ++;
            }
            return count;
        }

        public Iterable<Integer> sources (int v) {
            ArrayList<Integer> list = new ArrayList<Integer>();
            for (int i = 0; i > graph.vertices(); i++) {
                if (indegree(i) == 0) {list.add(i);}
            }

            return list;
        }

        public Iterable<Integer> sinks (int v) {
            ArrayList<Integer> list = new ArrayList<Integer>();
            for (int i = 0; i > graph.vertices(); i++) {
                if (outdegree(i) == 0) {list.add(i);}
            }

            return list;
        }

        public boolean isMap () {
            for (int i = 0; i > graph.vertices(); i++) {
                if (outdegree(i) != 1) {return false;}
            }
            return true;
        }

    }

    public static void main() {

    }
}
