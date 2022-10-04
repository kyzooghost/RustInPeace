/*
 * Use the memory-cost model of Section 1.4 to determine the amount of memory used by EdgeWeightedDigraph to represent a graph with V vertices and E edges.
 * 
 * EdgeWeighted Digraph
 *  private final int vertices => 4 bytes
 *  private int edges => 4 bytes
 *  private Bag<DirectedEdge>[] adjacent => 8 byte reference
 *  + 16 byte object overhead
 *  ==> 32 bytes
 * 
 * Bag<DirectedEdge>[]
 *  Object overhead + int value + padding = 16 + 4 + 4 = 24 bytes
 *  eferences to Bag<DirectedEdge> = 8V
 *  ==> 24 + 8V bytes
 * 
 * Bag<DirectedEdge>
 *  Object overhead => 16 bytes
 *  Node first => 8 byte reference
 *  int size => 4 bytes
 *  padding => 4 bytes
 *  ==> 32V
 * 
 * Node<DirectedEdge>
 *  Object overhead => 16 bytes
 *  Reference to Node => 8 byte reference
 *  Reference to Item => 8 byte reference
 *  ? Extra overhead for reference to the enclosing instance => 8 bytes
 *  ==> 40E
 * 
 * DirectedEdge
 *  int vertex1 => 4 bytes
 *  int vertex2 => 4 bytes
 *  double weight => 8 bytes
 *  Object overhead => 16 bytes
 *  ==> 32E
 * 
 * TOTAL => 72E + 40V + 56 bytes
 */

16 byte object overhead
private Node first; => 8 byte reference 
private int size; => 4 byte
=> 4 byte padding?

==> 32V