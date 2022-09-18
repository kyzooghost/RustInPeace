// Determine the amount of memory used by Graph to represent a graph with V vertices and E edges, using the memory-cost model of Section 1.4.

// 561

// private final int vertices; => 4 bytes
// protected int edges; => 4 bytes
// private Bag<Integer>[] adjacent; => Array of V bags, 2E nodes
// 8 + 8V + 16E ?

/* 

Bag

private Node first;
private int size;

private class Node {
    Item item;
    Node next;
}


INTEGER
- Object overhead = 16 bytes
- int value - 4 bytes
- padding - 4 bytes
- TOTAL 24 bytes

NODE
- 16 byte object overhead + 8 byte reference to item + 8 byte reference to next
- TOTAL 40 bytes

BAG
- 16 (object overhead) + 8 (Node reference) + 4 (int) + 4 (padding) = 32 byte overhead
- N Nodes + N Integers = 64N

GRAPH
- V BAGS + 2E NODES = Graph Overhead + Bag Array Overhead + V BAGS containing 2E Nodes
= 32 + (24 + 8V) + (128E + 32V)
= 128E + 40V + 56 bytes

*/

public class _4_1_27 {
    
}
