// 4.1.3

// Create a copy constructor for Graph that takes as input a graph G and creates and initializes a new copy of the graph. Any changes a client makes to G should not affect the newly created graph.

// Provide array of tuples representing edge => Construct graph

import java_utils.Bag;
import java_utils.Stack;
import java_utils.Graph;

@SuppressWarnings("unchecked")
public class _4_1_3 {

   public class GraphA {
      // Instance variables
      private final int vertices;
      private int edges;
      private Bag<Integer>[] adjacent;

      // Constructor
      public GraphA(int vertices) {
         this.vertices = vertices;
         this.edges = 0;
         adjacent = (Bag<Integer>[]) new Bag[vertices];
         for (int i = 0; i < vertices; i++) {
             adjacent[i] = new Bag<>();
         }
      }

      // Public getters
      public int vertices() {return vertices;}
      public int edges() {return edges;}
      public Iterable<Integer> adjacent(int vertex) {return adjacent[vertex];}

      public String toString() {
         StringBuilder stringBuilder = new StringBuilder();

         for (int vertex = 0; vertex < vertices(); vertex++) {
             stringBuilder.append(vertex).append(": ");

             for (int neighbor : adjacent(vertex)) {
                 stringBuilder.append(neighbor).append(" ");
             }
             stringBuilder.append("\n");
         }

         return stringBuilder.toString();
      }

      // Modifier
      public void addEdge(int vertex1, int vertex2) {
         adjacent[vertex1].add(vertex2);
         adjacent[vertex2].add(vertex1);
         edges++;
     }
   }

   public class CopyGraph {
      private final int vertices;
      private int edges;
      private Bag<Integer>[] adjacent;

      public CopyGraph(Graph graph) {
          if (graph == null) {
              vertices = 0;
          } else {
              this.vertices = graph.vertices();
              this.edges = graph.edges();
              adjacent = (Bag<Integer>[]) new Bag[vertices];

              for (int i = 0; i < vertices; i++) {
                  adjacent[i] = new Bag<>();
              }

              for (int vertex = 0; vertex < graph.vertices(); vertex++) {
                  // Reverse so that adjacency list is in the same order as original
                  Stack<Integer> stack = new Stack<>();
                  for (int neighbor : graph.getAdjacencyList()[vertex]) {
                      stack.push(neighbor);
                  }
                  for (int neighbor : stack) {
                      adjacent[vertex].add(neighbor);
                  }
              }
          }
      }

      public int vertices() {
          return vertices;
      }

      public int edges() {
          return edges;
      }

      public void addEdge(int vertex1, int vertex2) {
          adjacent[vertex1].add(vertex2);
          adjacent[vertex2].add(vertex1);
          edges++;
      }

      public Iterable<Integer> adjacent(int vertex) {
          return adjacent[vertex];
      }

      @Override
      public String toString() {
          StringBuilder stringBuilder = new StringBuilder();

          for (int vertex = 0; vertex < vertices(); vertex++) {
              stringBuilder.append(vertex).append(": ");

              for (int neighbor : adjacent(vertex)) {
                  stringBuilder.append(neighbor).append(" ");
              }
              stringBuilder.append("\n");
          }

          return stringBuilder.toString();
      }
   }

   public static void main(String[] args) {
      _4_1_3 exercise = new _4_1_3();
      GraphA graphA = exercise.new GraphA(12);
      graphA.addEdge(8, 4);
      graphA.addEdge(2, 3);
      graphA.addEdge(1, 11);
      graphA.addEdge(0, 6);
      graphA.addEdge(3, 6);
      graphA.addEdge(10, 3);
      graphA.addEdge(7, 11);
      graphA.addEdge(7, 8);
      graphA.addEdge(11, 8);
      graphA.addEdge(2, 0);
      graphA.addEdge(6, 2);
      graphA.addEdge(5, 2);
      graphA.addEdge(5, 10);
      graphA.addEdge(8, 1);
      graphA.addEdge(4, 1);
      graphA.addEdge(5, 0);

      System.out.println("edges: " + graphA.edges());
      System.out.println(graphA.toString());

      Graph graph = new Graph(12);
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
      graph.addEdge(8, 1);
      graph.addEdge(4, 1);
      graph.addEdge(5, 0);
      CopyGraph copyGraph = exercise.new CopyGraph(graph);
      // System.out.println(copyGraph.edges());
      System.out.println(copyGraph.toString());
   }
 }