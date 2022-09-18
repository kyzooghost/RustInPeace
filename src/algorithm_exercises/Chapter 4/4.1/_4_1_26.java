// Write a SymbolGraph client like DegreesOfSeparation that uses depth-first search instead of breadth-first search to find paths connecting two performers, producing output like that shown on the facing page.

// p555 - 561

import java_utils.SymbolGraph;
import java_utils.Graph;
import java_utils.DepthFirstPaths;

public class _4_1_26 {

    public class DegreesOfSeparationDFS {
        private SymbolGraph symbolGraph;
        private DepthFirstPaths paths;

        public DegreesOfSeparationDFS(SymbolGraph symbolGraph, String source) throws Exception {
            if (!symbolGraph.contains(source)) {throw new Exception("source not in database");}
            this.symbolGraph = symbolGraph;
            Graph graph = symbolGraph.graph();
            int index = symbolGraph.index(source);
            paths = new DepthFirstPaths(graph, index);
        }

        public void printPath(String query) throws Exception {
            if (!symbolGraph.contains(query)) {throw new Exception("query not in database");}
            int index = symbolGraph.index(query);
            Iterable<Integer> path = paths.pathTo(index);
            for (int i : path) {
                System.out.println(symbolGraph.name(i));

            }

        }
    }
    public static void main(String[] args) {
        _4_1_26 exercise = new _4_1_26();
        String FILE_PATH = "./movies.txt";
        String SEPARATOR = "/";
        SymbolGraph symbolGraph = new SymbolGraph(FILE_PATH, SEPARATOR);
        try {
            DegreesOfSeparationDFS query = exercise.new DegreesOfSeparationDFS(symbolGraph, "Bacon, Kevin");
            query.printPath("Eccleston, Christopher");
        } catch (Exception e) {
            System.out.println("Error!");
        }
    }
}
