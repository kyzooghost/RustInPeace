// Digraph enumeration. 

// Show that the number of different V-vertex digraphs with no parallel edges is 2^(V^2) . 
// (How many digraphs are there that contain V vertices and E edges?) 
// Then compute an upper bound on the percentage of 20-vertex digraphs that could ever be examined by any computer, under the assumptions that every electron in the universe examines a digraph every nanosecond, that the universe has fewer than 1080 electrons, and that the age of the universe will be less than 1020 years.

// Digraph with V vertices, and no parallel edges
// Each node can either have or not have an outedge to every other node
// This choice is made V^2 times (V nodes, making V decisions each)
// There are two choices to make in each decision, hence 2^(V^2)?

// Digraphs with V vertices and E edges = ((V^2) choose E) = (# of node pairs) choose E

// 20-vertex digraphs => 2^(20^2) => 2^400 20-vertex digraphs with no parallel edge
// # of digraphs examined = 1080 * 1020 years / nanoseconds = 1080 / 1020 * 31,536,000 * 10^9 = 1.08 * 1.02 * 3.1536 * 10^(9+3+3+7) = 3.474 * 10^22 ~= 2^75
// 1 / 2^325 lmao



public class _4_2_27 {
    
}
