// Add a method hasEdge() to Digraph which takes two int arguments v and w and returns true if the graph has an edge v->w, false otherwise.

import java_utils.Digraph;

public class _4_2_4 {
    
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
    }

    public static void main(String[] args) {
        _4_2_4 exercise = new _4_2_4();
        DigraphPlus graph = exercise.new DigraphPlus(12);
        graph.addEdge(8, 4);
        graph.addEdge(2, 3);
        graph.addEdge(1, 11);
        graph.addEdge(0, 6);
        graph.addEdge(3, 6);
        graph.addEdge(10, 3);
        graph.addEdge(7, 11);
        graph.addEdge(7, 8);
        graph.addEdge(11, 8);
        graph.addEdge(2, 0);
        graph.addEdge(6, 2);
        graph.addEdge(5, 2);
        graph.addEdge(5, 10);
        graph.addEdge(3, 10);
        graph.addEdge(8, 1);
        graph.addEdge(4, 1);

        System.out.println(graph.hasEdge(8, 4));  
    }
}
