package java_utils;

public class Topological {
    private Iterable<Integer> order;

    public Topological(Digraph G) {
        // First DFS to check for cycle
        DirectedCycle cyclefinder = new DirectedCycle(G);

        // Second DFS to find reverse post order
        if (!cyclefinder.hasCycle()) {
            DepthFirstOrder dfs = new DepthFirstOrder(G);
            order = dfs.reversePost(); // Reverse post order == topological sort
        }
    }

    public Topological(EdgeWeightedDigraph edgeWeightedDigraph) {
        EdgeWeightedDirectedCycle cycleFinder = new EdgeWeightedDirectedCycle(edgeWeightedDigraph);

        if (!cycleFinder.hasCycle()) {
            DepthFirstOrder depthFirstOrder = new DepthFirstOrder(edgeWeightedDigraph);
            order = depthFirstOrder.reversePost();
        }
    }

    public Iterable<Integer> order() {return order;}
    public boolean isDAG() {return order == null;}
    public void printOrder() {
        StringBuilder string = new StringBuilder();
        for (int i : order) {
            string.append(i).append(" ");
        }
        System.out.println(string);
    }
}
