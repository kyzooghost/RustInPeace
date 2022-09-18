// 4.1.11 - p558

// Show, in the style of the figure on page 533, a detailed trace of the call bfs(0) for the graph built by Graph’s input stream constructor for the file tinyGex2.txt (see Exercise 4.1.2). Also, draw the tree represented by edgeTo[].

// Adjacency vectors
// 0: 5 2 6 
// 1: 4 8 11 
// 2: 5 6 0 3 
// 3: 10 6 2 
// 4: 1 8 
// 5: 0 10 2 
// 6: 2 3 0 
// 7: 8 11 
// 8: 1 11 7 4 
// 9: 
// 10: 5 3 
// 11: 8 7 1 

// bfs(0)
// marked[0] = true
// marked[0, 5, 2, 6] = true, edgeTo[5, 2, 6] = 0, queue = [5, 2, 6]
// marked[0, 5, 2, 6, 10] = true, edgeTo[5, 2, 6] = 0, edge[10] = 5, queue = [2, 6, 10]
// marked[0, 2, 3, 5, 6, 10] = true, edgeTo[5, 2, 6] = 0, edge[10] = 5, edge[3] = 2, queue = [6, 10, 3]
// marked[0, 2, 3, 5, 6, 10] = true, edgeTo[5, 2, 6] = 0, edge[10] = 5, edge[3] = 2

public class _4_1_11 {
    
}
