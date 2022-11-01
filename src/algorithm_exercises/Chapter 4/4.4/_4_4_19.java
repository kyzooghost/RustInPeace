/*
 * p686
 * Find the lowest-weight cycle (best arbitrage opportunity) in the example shown in the text.
 * 
 * So we can represent the entire currency exchange table as a complete edge-weighted digraph.
 * If you replace each exchange rate 'x' with '-ln(x)', any negative cycle is an arbitrage opportunity
 * We normally add edge weights to find the length of a cycle, whereas we normally multiple exchange rates.
 * 'x0 * x1 * x2 > 1'
 * => `ln(x0 * x1 * x2) > ln (1)
 * => ln(x0) + ln(x1) + ln(x2) > 0
 * => - ln(x0) - ln(x1) - ln(x2) < 0
 * 
 * The original EdgeWeightedDirectedCycle implementation stops when it finds a single negative cycle. We have modified it into a EdgeWeightedDirectedCycles class that does not stop on finding a single negative cycle.
 * 
 * BellmanFordSPModified is modified only to use EdgeWeightedDirectedCycles class, and enable storing multiple cycles rather than a single cycle.
 * 
 * ArbitrageFinger will create a BellmanFordSPModified instance for each given vertex, and collect the negative cycles found in each one.
 */

import java_utils.DirectedEdge;
import java_utils.DirectedEdgePath;
import java_utils.EdgeWeightedDigraph;
import java_utils.HashSet;
import java_utils.Stack;
import java_utils.Queue;
import java.util.HashMap;
import java.util.ArrayList;
import java.util.Collections;

public class _4_4_19 {

    // Modified EdgeWeightedDirectedCycle class, to find multiple cycles rather than stop at finding one cycle.
    public class EdgeWeightedDirectedCycles {

        private boolean visited[];
        private DirectedEdge[] edgeTo;
        private ArrayList<DirectedEdgePath> cycles; // Collection of cycles
        private boolean[] onStack; // vertices on recursive call stack
    
        public EdgeWeightedDirectedCycles(EdgeWeightedDigraph edgeWeightedDigraph) {
            this.onStack = new boolean[edgeWeightedDigraph.vertices()];
            this.edgeTo = new DirectedEdge[edgeWeightedDigraph.vertices()];
            this.visited = new boolean[edgeWeightedDigraph.vertices()];
            this.cycles = new ArrayList<DirectedEdgePath>();
    
            for (int vertex = 0; vertex < edgeWeightedDigraph.vertices(); vertex++) {
                if (!visited[vertex]) {
                    dfs(edgeWeightedDigraph, vertex);
                }
            }
        }
    
        private void dfs(EdgeWeightedDigraph edgeWeightedDigraph, int vertex) {
            onStack[vertex] = true;
            visited[vertex] = true;
    
            for (DirectedEdge edge : edgeWeightedDigraph.adjacent(vertex)) {
                int neighbor = edge.to();
    
                // Classic DFS
                if (!visited[neighbor]) {
                    edgeTo[neighbor] = edge;
                    dfs(edgeWeightedDigraph, neighbor);
                // If neighbor is on stack, it means we have found a cycle
                } else if (onStack[neighbor]) {
                    DirectedEdgePath cycle = new DirectedEdgePath(neighbor);
                    Stack<DirectedEdge> reverseCycle = new Stack<DirectedEdge>();
                    // This is the last edge in the cycle
                    DirectedEdge edgeInCycle = edge;
                    // Work backwards until we find the neighbor in our edge-weighted digraph.
                    while (edgeInCycle.from() != neighbor) {
                        reverseCycle.push(edgeInCycle);
                        edgeInCycle = edgeTo[edgeInCycle.from()];
                    }
                    reverseCycle.push(edgeInCycle);

                    while (!reverseCycle.isEmpty()) {
                        cycle = new DirectedEdgePath(cycle, reverseCycle.pop());
                    }

                    cycles.add(cycle);

                    // Unsure if we should return here.
                    // If we find a cycle, I don't want to dfs() into the neighbor, I want to record the cycle, and then I want to keep going and loop into the next adjacent edge.
                    // return;
                }

                // Else we have already visited the neighbor, and this previsited neighbor isn't also on the dfs stack
            }
    
            onStack[vertex] = false;
        }
    
        public boolean hasCycle() {
            return this.cycles.isEmpty() != false;
        }
    
        public ArrayList<DirectedEdgePath> cycles() {
            return cycles;
        }
    }

    public class BellmanFordSPModified {

        private double[] distTo;               // length of path to vertex
        private DirectedEdge[] edgeTo;         // last edge on path to vertex
        private boolean[] onQueue;             // is this vertex on the queue?
        private Queue<Integer> queue;          // vertices being relaxed
        private int callsToRelax;              // number of calls to relax()
        private ArrayList<DirectedEdgePath> cycles;  // are negative cycles in edgeTo[], return it
    
        public BellmanFordSPModified(EdgeWeightedDigraph edgeWeightedDigraph, int source) {
            distTo = new double[edgeWeightedDigraph.vertices()];
            edgeTo = new DirectedEdge[edgeWeightedDigraph.vertices()];
            onQueue = new boolean[edgeWeightedDigraph.vertices()];
            queue = new Queue<>();
    
            for (int vertex = 0; vertex < edgeWeightedDigraph.vertices(); vertex++) {
                distTo[vertex] = Double.POSITIVE_INFINITY;
            }
    
            distTo[source] = 0;
            queue.enqueue(source);
            onQueue[source] = true;
    
            while (!queue.isEmpty() && !hasNegativeCycle()) {
                int vertex = queue.dequeue();
                onQueue[vertex] = false;
                relax(edgeWeightedDigraph, vertex);
                // System.out.println(vertex + " dequeued");
                // printEdgeTo();
                // printQueue();
                // System.out.println();
            }
        }
    
        private void relax(EdgeWeightedDigraph edgeWeightedDigraph, int vertex) {
    
            for (DirectedEdge edge : edgeWeightedDigraph.adjacent(vertex)) {
                int neighbor = edge.to();
    
                if (distTo[neighbor] > distTo[vertex] + edge.weight()) {
                    distTo[neighbor] = distTo[vertex] + edge.weight();
                    edgeTo[neighbor] = edge;
    
                    if (!onQueue[neighbor]) {
                        queue.enqueue(neighbor);
                        onQueue[neighbor] = true;
                    }
                }
    
                if (callsToRelax++ % edgeWeightedDigraph.vertices() == 0) {
                    findNegativeCycle();
                }
            }
        }
    
        public double distTo(int vertex) {
            return distTo[vertex];
        }
    
        public boolean hasPathTo(int vertex) {
            return distTo[vertex] < Double.POSITIVE_INFINITY;
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
    
        private void findNegativeCycle() {
            int vertices = edgeTo.length;
            EdgeWeightedDigraph shortestPathsTree = new EdgeWeightedDigraph(vertices);
    
            for (int vertex = 0; vertex < vertices; vertex++) {
                if (edgeTo[vertex] != null) {
                    shortestPathsTree.addEdge(edgeTo[vertex]);
                }
            }
    
            EdgeWeightedDirectedCycles edgeWeightedCycleFinder = new EdgeWeightedDirectedCycles(shortestPathsTree);
            this.cycles = edgeWeightedCycleFinder.cycles();
        }
    
        public boolean hasNegativeCycle() {
            return cycles != null && cycles.size() > 0;
        }
    
        public ArrayList<DirectedEdgePath> cycles() {
            return cycles;
        }

        public void printEdgeTo() {
            for(int i = 0; i < distTo.length; i++) {
                if (edgeTo[i] != null) {
                    System.out.println("   edge: " + edgeTo[i].from() + "->" + edgeTo[i].to() + ", distTo: " + String.format("%.2f", distTo[i]));
                } else {
                    System.out.println("   edge: " + edgeTo[i] + ", distTo: " + String.format("%.2f", distTo[i]));
                }
            }
        }

        public void printQueue() {
            StringBuilder str = new StringBuilder();
            for (int vertex : this.queue) {
                str.append(vertex).append(" ");
            }
            System.out.println("   queue:" + str);
        }
    }

    public class ArbitrageFinder {
        private EdgeWeightedDigraph graph;
        private ArrayList<DirectedEdgePath> cycles;

        ArbitrageFinder(EdgeWeightedDigraph graph_) {
            this.graph = graph_;
            this.cycles = new ArrayList<DirectedEdgePath>();
            ArrayList<DirectedEdgePath> unfilteredCycles = new ArrayList<DirectedEdgePath>();

            for (int vertex = 0; vertex < graph_.vertices(); vertex++) {
                BellmanFordSPModified spt = new BellmanFordSPModified(graph_, vertex);
                if (spt.cycles() != null) {
                    unfilteredCycles.addAll(spt.cycles());
                }
            }

            if (unfilteredCycles.size() > 0) {
                HashSet<Double> cycleLengthSet = new HashSet<Double>();
                unfilteredCycles.forEach((cycle) -> {
                    if (!cycleLengthSet.contains(cycle.weight())) {
                        cycleLengthSet.add(cycle.weight());
                        this.cycles.add(cycle);
                    }
                });

                Collections.sort(this.cycles);
            }
        }

        public boolean hasArbitrage() {
            return this.cycles != null;
        }

        public ArrayList<DirectedEdgePath> getArbitrages() {
            return this.cycles;
        }

        public DirectedEdgePath getBestArbitrage() {
            return this.cycles.get(0);
        }
    }

    public class CyclePrinter {
        
        public static void printCycles(ArrayList<DirectedEdgePath> cycles) {
            if (cycles == null || cycles.size() == 0) {
                return;
            }

            for (DirectedEdgePath cycle : cycles) {
                StringBuilder str = new StringBuilder();
                int counter = 0;
                for (DirectedEdge edge : cycle.getPath()) {
                    if (counter == 0) {
                        str.append(edge.from()).append("->").append(edge.to());
                    } else {
                        str.append("->").append(edge.to());
                    }
                    counter++;
                }
                System.out.println("Cycle: " + str);
            }
        }
    }

    public class ArbitragePrinter {
        
        public static void printArbitrages(ArrayList<DirectedEdgePath> cycles, HashMap<Integer, String> nodeToCurrency) {
            if (cycles == null || cycles.size() == 0) {
                return;
            }

            for (DirectedEdgePath cycle : cycles) {
                StringBuilder str = new StringBuilder();
                int counter = 0;
                for (DirectedEdge edge : cycle.getPath()) {
                    if (counter == 0) {
                        str.append(nodeToCurrency.get(edge.from())).append("->").append(nodeToCurrency.get(edge.to()));
                    } else {
                        str.append("->").append(nodeToCurrency.get(edge.to()));
                    }
                    counter++;
                }
                System.out.println("Arbitrage: " + str);
            }
        }
    }

    public static void main(String[] args) {
        _4_4_19 exercise = new _4_4_19();
        CyclePrinter cyclePrinter = exercise.new CyclePrinter();
        ArbitragePrinter arbitragePrinter = exercise.new ArbitragePrinter();

        // Test
        EdgeWeightedDigraph graph0 = new EdgeWeightedDigraph(8);
        graph0.addEdge(new DirectedEdge(4, 5, 0.35));
        graph0.addEdge(new DirectedEdge(5, 4, -0.66));
        graph0.addEdge(new DirectedEdge(4, 7, 0.37));
        graph0.addEdge(new DirectedEdge(5, 7, 0.28));
        graph0.addEdge(new DirectedEdge(7, 5, 0.28));
        graph0.addEdge(new DirectedEdge(5, 1, 0.32));
        graph0.addEdge(new DirectedEdge(0, 4, 0.38));
        graph0.addEdge(new DirectedEdge(0, 2, 0.26));
        graph0.addEdge(new DirectedEdge(7, 3, 0.39));
        graph0.addEdge(new DirectedEdge(1, 3, 0.29));
        graph0.addEdge(new DirectedEdge(2, 7, 0.34));
        graph0.addEdge(new DirectedEdge(6, 2, 0.40));
        graph0.addEdge(new DirectedEdge(3, 6, 0.52));
        graph0.addEdge(new DirectedEdge(6, 0, 0.58));
        graph0.addEdge(new DirectedEdge(6, 4, 0.93));

        BellmanFordSPModified spt0 = exercise.new BellmanFordSPModified(graph0, 0);
        cyclePrinter.printCycles(spt0.cycles());

        HashMap<String, Integer> currencyToNode = new HashMap<String, Integer>();
        currencyToNode.put("USD", 0);
        currencyToNode.put("EUR", 1);
        currencyToNode.put("GBP", 2);
        currencyToNode.put("CHF", 3);
        currencyToNode.put("CAD", 4);

        HashMap<Integer, String> nodeToCurrency = new HashMap<Integer, String>();
        for (HashMap.Entry<String, Integer> entry : currencyToNode.entrySet()) {
            nodeToCurrency.put(entry.getValue(), entry.getKey());
        }

        // Graph edges from table on p679
        EdgeWeightedDigraph graph = new EdgeWeightedDigraph(5);
        graph.addEdge(new DirectedEdge(currencyToNode.get("USD"), currencyToNode.get("EUR"), -Math.log(0.741)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("USD"), currencyToNode.get("GBP"), -Math.log(0.657)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("USD"), currencyToNode.get("CHF"), -Math.log(1.061)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("USD"), currencyToNode.get("CAD"), -Math.log(1.005)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("EUR"), currencyToNode.get("USD"), -Math.log(1.349)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("EUR"), currencyToNode.get("GBP"), -Math.log(0.888)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("EUR"), currencyToNode.get("CHF"), -Math.log(1.433)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("EUR"), currencyToNode.get("CAD"), -Math.log(1.366)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("GBP"), currencyToNode.get("USD"), -Math.log(1.521)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("GBP"), currencyToNode.get("EUR"), -Math.log(1.126)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("GBP"), currencyToNode.get("CHF"), -Math.log(1.614)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("GBP"), currencyToNode.get("CAD"), -Math.log(1.538)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("CHF"), currencyToNode.get("USD"), -Math.log(0.942)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("CHF"), currencyToNode.get("EUR"), -Math.log(0.698)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("CHF"), currencyToNode.get("GBP"), -Math.log(0.619)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("CHF"), currencyToNode.get("CAD"), -Math.log(0.953)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("CAD"), currencyToNode.get("USD"), -Math.log(0.995)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("CAD"), currencyToNode.get("EUR"), -Math.log(0.732)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("CAD"), currencyToNode.get("GBP"), -Math.log(0.650)));
        graph.addEdge(new DirectedEdge(currencyToNode.get("CAD"), currencyToNode.get("CHF"), -Math.log(1.049)));
        ArbitrageFinder arbitrageFinder = exercise.new ArbitrageFinder(graph);
        arbitragePrinter.printArbitrages(arbitrageFinder.getArbitrages(), nodeToCurrency);
    }
}