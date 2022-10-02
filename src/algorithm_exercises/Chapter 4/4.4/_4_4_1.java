// Adding a constant to every edge weight does not change the solu- tion to the single-source shortest-paths problem.

// False, book talked about case where some edge weights are negative, we cannot just find the SPT by adding a constant such that all edge weights are now positive because the SPT is mutated in the process
// Adding a constant to every edge weight more heavily 'penalises' the paths with more edges. So an SPT can change to one with fewer edges.