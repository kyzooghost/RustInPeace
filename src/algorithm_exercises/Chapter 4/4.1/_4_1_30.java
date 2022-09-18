/*
p562

0-1 0-2 0-3 1-3 1-4 2-5 2-9 3-6 4-7 4-8 5-8 5-9 6-7 6-9 7-8
0-1 0-2 0-3 1-3 0-3 2-5 5-6 3-6 4-7 4-8 5-8 5-9 6-7 6-9 8-8
0-1 1-2 1-3 0-3 0-4 2-5 2-9 3-6 4-7 4-8 5-8 5-9 6-7 6-9 7-8
4-1 7-9 6-2 7-3 5-0 0-2 0-8 1-6 3-9 6-3 2-8 1-5 9-8 4-5 4-7

Which of these graphs have Euler cycles (cycles that visit each edge exactly once)? 
Which of them have Hamilton cycles (cycles that visit each vertex exactly once)?

// Define a client to find each cycle

Hamilton
0-1
 */

import java_utils.Graph;
import java_utils.DepthFirstPaths;
import java_utils.Set;
import java.util.ArrayList;

@SuppressWarnings("unchecked")
public class _4_1_30 {

    class Pair {
        private int a;
        private int b;

        public Pair (int a, int b) {
            this.a = a;
            this.b = b;
        }

        public boolean equals (Pair pair) {
            if ( (pair.a == a && pair.b == b) || (pair.a == b && pair.b == a) ) {return true;}
            return false;
        }
    }

    class PairSet {
        private ArrayList<Pair> pairArray;

        public PairSet () {
            this.pairArray = new ArrayList<Pair>();
        }

        public void add(Pair pair) {
            pairArray.add(pair);
        }

        public int size() {return pairArray.size();}

        public boolean contains (Pair pair) {
            for (Pair i : pairArray) {
                if (i.equals(pair)) {return true;}
            }
            return false;
        }
    }

    public class CycleQuery {
        private Graph graph;
        private DepthFirstPaths[] paths;

        public CycleQuery (String edges) {
            graph = new Graph(10);
            String[] pair_array = edges.split(" ");
            for (int i = 0; i < pair_array.length; i++) {
                String[] pair = pair_array[i].split("-");
                graph.addEdge(Integer.parseInt(pair[0]), Integer.parseInt(pair[1]));
            }

            paths = new DepthFirstPaths[graph.vertices()];
            for (int i = 0; i < graph.vertices(); i++) {
                paths[i] = new DepthFirstPaths(graph, i);
            }
        }

        public int vertices () {return graph.vertices();}
        public int edges () {return graph.edges();}

        // Cycle that visit each vertex once
        public boolean hasHamiltonCycle() {
            for (int i = 0; i < vertices(); i++) {
                if (!paths[i].hasPathTo(i)) {continue;} // Continue if no cycle
                Iterable<Integer> cyclePath = paths[i].pathTo(0);
                Set set = new Set();

                StringBuilder string = new StringBuilder();
                for (int j : cyclePath) {
                    string.append(j).append("-");

                    if (set.contains(j)) {break;}
                    else {set.add(j);}
                }

                System.out.println(string);
                System.out.println(i + " - " + set.size());

                if (set.size() == vertices()) {return true;}
            }

            return false;
        }

        // Cycle that visit each edge once
        public boolean hasEulerCycle() {
            for (int i = 0; i < vertices(); i++) {
                if (!paths[i].hasPathTo(i)) {continue;} // Continue if no cycle
                Iterable<Integer> cyclePath = paths[i].pathTo(0);

                PairSet set = new PairSet();

                int previousVertex = Integer.MAX_VALUE;

                for (int vertex : cyclePath) {
                    if (previousVertex != Integer.MAX_VALUE) {
                        Pair pair = new Pair(previousVertex, vertex);
                        if (set.contains(pair)) {break;}
                        else {set.add(pair);}
                    }
                    previousVertex = vertex;
                }

                if (set.size() == edges()) {return true;}
            }

            return false;
        }
    }

    public static void main(String[] args) {
        // Construct graphs
        _4_1_30 exercise = new _4_1_30();

        String g1 = "0-1 0-2 0-3 1-3 1-4 2-5 2-9 3-6 4-7 4-8 5-8 5-9 6-7 6-9 7-8";
        String g2 = "0-1 0-2 0-3 1-3 0-3 2-5 5-6 3-6 4-7 4-8 5-8 5-9 6-7 6-9 8-8";
        String g3 = "0-1 1-2 1-3 0-3 0-4 2-5 2-9 3-6 4-7 4-8 5-8 5-9 6-7 6-9 7-8";
        String g4 = "4-1 7-9 6-2 7-3 5-0 0-2 0-8 1-6 3-9 6-3 2-8 1-5 9-8 4-5 4-7";

        CycleQuery query1 = exercise.new CycleQuery(g1);
        CycleQuery query2 = exercise.new CycleQuery(g2);
        CycleQuery query3 = exercise.new CycleQuery(g3);
        CycleQuery query4 = exercise.new CycleQuery(g4);

        System.out.println(query1.hasHamiltonCycle());
        System.out.println(query2.hasHamiltonCycle());
        System.out.println(query3.hasHamiltonCycle());
        System.out.println(query4.hasHamiltonCycle());

        // System.out.println(query1.hasEulerCycle());
        // System.out.println(query2.hasEulerCycle());
        // System.out.println(query3.hasEulerCycle());
        // System.out.println(query4.hasEulerCycle());

    }
}

// Lol, Eulerian cycle if all vertices have an even degree via theorem
// And code didn't find Hamilton cycle for graph 1, can adjust DepthFirstPaths to take longer path than that found by DFS. DFS finds a path, but not the longest path.