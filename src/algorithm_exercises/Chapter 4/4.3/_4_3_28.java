/*
 * Space-efficient data structures
 * 
 * Develop an implementation of the lazy version of Prim’s algorithm that saves space by using lower-level data structures for EdgeWeightedGraph and for MinPQ instead of Bag and Edge.
 * 
 * Estimate the amount of memory saved as a function of V and E, using the memory-cost model
 * 
 * Use MinPQ<double> rather than MinPQ<Edge>, we lose the edge vertices like this
 * Store adjacent edges in a double[][], where Edge(v1, v2, w) is stored as double[v1][v2] = weight & double[v2][v1] = weight
 * Reading https://github.com/reneargento/algorithms-sedgewick-wayne/blob/master/src/chapter4/section3/Exercise28_SpaceEfficientDataStructures.java though, we sacrificing huge time efficiency (and increasing code complexity significantly) for small space savings, because we need to find the vertices correlating to each weight within the double[][] data structure.
 */