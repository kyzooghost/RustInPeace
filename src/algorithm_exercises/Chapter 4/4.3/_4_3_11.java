// Determine the amount of memory used by EdgeWeightedGraph to represent a graph with V vertices and E edges, using the memory-cost model of Section 1.4.

/*

EdgeWeightedGraph
    private final int vertices; => 4 bytes
    private int edges; => 4 bytes
    private Bag<Edge>[] adjacent; => 8 byte reference
    + 16 byte object overhead

    ==> 32 bytes

Bag<Edge>[]
    Object overhead + int value + padding = 16 + 4 + 4 = 24 bytes
    References to Bag<Edge> = 8V

    ==> 24 bytes + 8V

Bag<Edge>
    16 byte object overhead
    private Node first; => 8 byte reference 
    private int size; => 4 byte
    => 4 byte padding?

    ==> 32V

Edge
    private final int vertex1; => 4 bytes
    private final int vertex2; => 4 bytes
    private final double weight; => 8 bytes
    Object overhead => 16 bytes

    ==> 32E

Node<Edge>
    Object overhead => 16 bytes
    Reference to Item => 8 bytes
    Reference to Node => 8 bytes
    ? Extra overhead for reference to the enclosing instance => 8 bytes
     
    ==> 40E * 2 (Node repeated twice, but not Edge)

Total
    32 + 24 + 8V + 32V + 32E + 80E = 56 bytes + 40V + 112E

 */

public class _4_3_11 {
    
}
