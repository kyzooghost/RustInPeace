// 4.1.9 - p558

// Show, in the style of the figure on page 533, a detailed trace of the call dfs(0) for the graph built by Graph’s input stream constructor for the file tinyGex2.txt (see Exercise 4.1.2). Also, draw the tree represented by edgeTo[].

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


// dfs(0)
// marked[0, 5]
// 0: 5
// 5: 0

    // edgeTo[5] = 0
    // dfs(5)
    // marked[0, 5, 10]
    // 0: 5
    // 5: 0, 10
    // 10: 5

        // edgeTo[10] = 5
        // dfs(10)

            // edgeTo[3] = 10
            // dfs(3)
            // marked[0, 3, 5, 10]
            // 0: 5
            // 3: 10
            // 5: 0, 10
            // 10: 5, 3

        // edgeTo[2] = 5
        // dfs(2)
        // marked[0, 2, 3, 5, 10]
        // 0: 5
        // 2: 5
        // 3: 10
        // 5: 0, 10, 2
        // 10: 5, 3

            // edgeTo[6] = 2
            // dfs(6)
            // marked[0, 2, 3, 5, 6, 10]
            // 0: 5
            // 2: 5, 6
            // 3: 10
            // 5: 0, 10, 2
            // 6: 2
            // 10: 5, 3

// edgeTo
// [0] = null
// [2] = 5
// [3] = 10
// [5] = 0
// [6] = 2
// [10] = 5