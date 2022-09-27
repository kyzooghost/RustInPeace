/*
 * Partitioning. 
 * 
 * Develop an implementation based on integrating Kruskal’s algorithm with quicksort partitioning (instead of using a priority queue) so as to check MST membership of each edge as soon as all smaller edges have been checked.
 *
 * Instead of using minPQ to sort edges by weight, use quickselect and iterate through the sorted array of Edges - O (N log N) - which is time complexity as using a minPQ
 * 
 * If we use quickselect, it is O(N ^ 2) average and O(N ^ 3) worst case because we need to use quick select N times.
 * 
 * However the Incremental Sorting algorithm detailed in https://www.researchgate.net/publication/242328466_Optimal_Incremental_Sorting describes a method to obtain the MST in O(V lg V) time, as opposed to O (E lg E) time with quicksort (which works well in dense graphs where there are much more edges than vertices).
 * 
 */

public class _4_3_42 {
    
}
