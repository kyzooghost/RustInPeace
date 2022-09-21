// Compute the memory usage of a Digraph with V vertices and E edges, under the memory cost model of Section 1.4.

/*

int vertices + int value = 4 + 4
object overhead = 16
reference to Bag<Integer>[] -> 8
=> 32 bytes

Bag<Integer>[]
- Object overhead + int value + padding = 16 + 4 + 4 = 24
- References to Bag<Integer> = 8V
- Bag - 32V + 64E


Total
=> 56 + 40V + 64E

 */
public class _4_2_18 {
    
}
