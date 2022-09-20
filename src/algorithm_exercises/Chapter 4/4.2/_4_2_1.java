// What is the maximum number of edges in a digraph with V vertices and no parallel edges? 
// What is the minimum number of edges in a digraph with V vertices, none of which are isolated?

// V = 2 => 2 edges
// V = 3 => 2 edges between each node = 2 (2 + 1)
// V = 4 => 2 (3 + 2 + 1) = 12
// V = 5 => 2 * (4 + 3 + 2 + 1) = 20
// Max # of edges = 2 * (V choose 2)

// Minimum number if no isolated vertex?
// V = 2 => 1 edge
// V = 3 => 2 edge
// V = 4 => 3 edge
// (V - 1)

public class _4_2_1 {
    
}
