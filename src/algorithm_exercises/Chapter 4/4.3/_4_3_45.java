/*
 * External MST. 
 * Describe how you would find the MST of a graph so large that only V edges can fit into main memory at once.
 * 
 * Assumption 1: V < E, by a large factor
 * Assumption 2: Assuming we can only process V edges into RAM, and store the remaining edges in hard drive (i.e. if we were using the EVM memory model, only V edges can be stored in memory and the remainder in storage).
 * 
 * We cannot use Kruskal's, as it requires a priority queue of size E.
 * Eager Prim's algorithm seems fit for the job - edgeTo, distTo and marked are all arrays of size V. The priority queue is also of size V.
 */
