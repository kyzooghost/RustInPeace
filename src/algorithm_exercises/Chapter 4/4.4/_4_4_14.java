/*
 * p686
 * Show the paths that would be discovered by the two strawman approaches described on page 668 for the example tinyEWDn.txt shown on that page.
 * 
 * ORIGINAL
 * 
 * 4->5 0.35 
 * 5->4 0.35 
 * 4->7 0.37 
 * 5->7 0.28 
 * 7->5 0.28 
 * 5->1 0.32 
 * 0->4 0.38 
 * 0->2 0.26 
 * 7->3 0.39 
 * 1->3 0.29 
 * 2->7 0.34 
 * 6->2 -1.20 
 * 3->6 0.52 
 * 6->0 -1.40 
 * 6->4 -1.25
 * 
 * edgeTo[0] = null
 * edgeTo[1] = 5
 * edgeTo[2] = 0
 * edgeTo[3] = 7
 * edgeTo[4] = 6
 * edgeTo[5] = 4
 * edgeTo[6] = 3
 * edgeTo[7] = 2
 * 
 * distTo[0] = 0
 * distTo[1] = 0.93
 * distTo[2] = 0.26
 * distTo[3] = 0.99
 * distTo[4] = 0.26
 * distTo[5] = 0.61
 * distTo[6] = 1.51
 * distTo[7] = 0.60
 * 
 * 
 * STRAWMAN 1
 * 4->5 1.75
 * 5->4 1.75
 * 4->7 1.77
 * 5->7 1.68
 * 7->5 1.68
 * 5->1 1.72
 * 0->4 1.78
 * 0->2 1.66
 * 7->3 1.79
 * 1->3 1.69
 * 2->7 1.74
 * 6->2 0.2
 * 3->6 1.92
 * 6->0 0
 * 6->4 0.15
 * 
 * edgeTo[0] = null
 * edgeTo[1] = 5
 * edgeTo[2] = 0
 * edgeTo[3] = 7
 * edgeTo[4] = 0
 * edgeTo[5] = 4
 * edgeTo[6] = 3
 * edgeTo[7] = 2
 * 
 * distTo[0] = 0
 * distTo[1] = 5.25
 * distTo[2] = 1.66
 * distTo[3] = 5.19
 * distTo[4] = 1.78
 * distTo[5] = 3.53
 * distTo[6] = 7.11
 * distTo[7] = 3.4
 * 
 * 
 * STRAWMAN II
 * 
 * 4->5 0.35 
 * 5->4 0.35 
 * 4->7 0.37 
 * 5->7 0.28 
 * 7->5 0.28 
 * 5->1 0.32 
 * 0->4 0.38 
 * 0->2 0.26 
 * 7->3 0.39 
 * 1->3 0.29 
 * 2->7 0.34 
 * 6->2 -1.20 
 * 3->6 0.52 
 * 6->0 -1.40 
 * 6->4 -1.25
 * 
 * edgeTo[0] = null
 * edgeTo[1] = 5
 * edgeTo[2] = 0
 * edgeTo[3] = 7
 * edgeTo[4] = 6
 * edgeTo[5] = 4
 * edgeTo[6] = 3
 * edgeTo[7] = 2
 * 
 * distTo[0] = 0
 * distTo[1] = 1.05
 * distTo[2] = 0.26
 * distTo[3] = 0.99
 * distTo[4] = 0.26
 * distTo[5] = 0.73
 * distTo[6] = 1.51
 * distTo[7] = 0.60
 * 
 * Issue with Strawman II - Need to 'follow the chain' after negative edge applied in relax operation, whereas Dijkstra's doesn't allow this?
 */


public class _4_4_14 {
    
}