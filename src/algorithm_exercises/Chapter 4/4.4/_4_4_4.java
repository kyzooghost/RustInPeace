/*
 * Draw the SPT for source 0 of the edge-weighted digraph obtained by deleting vertex 7 from tinyEWD.txt (see page 644), and give the parent-link representation of the SPT. Answer the question for the same graph with all edge reversed.
 */

/*
 * 4 5 0.35
 * 5 4 0.35
 * 5 1 0.32
 * 0 4 0.38
 * 0 2 0.26
 * 1 3 0.29
 * 6 2 0.40
 * 3 6 0.52
 * 6 0 0.58
 * 6 4 0.93
 * 
 * edgeTo[0] = null
 * edgeTo[1] = 5
 * edgeTo[2] = 0
 * edgeTo[3] = 1
 * edgeTo[4] = 0
 * edgeTo[5] = 4
 * edgeTo[6] = 3
 * 
 * distTo[0] = 0
 * distTo[1] = 1.05
 * distTo[2] = 0.26
 * distTo[3] = 1.34
 * distTo[4] = 0.38
 * distTo[5] = 0.73
 * distTo[6] = 1.86
 */

/*
 * 5 4 0.35
 * 4 5 0.35
 * 1 5 0.32
 * 4 0 0.38
 * 2 0 0.26
 * 3 1 0.29
 * 2 6 0.40
 * 6 3 0.52
 * 0 6 0.58
 * 4 6 0.93
 * 
 * edgeTo[0] = null
 * edgeTo[1] = 3
 * edgeTo[2] = 
 * edgeTo[3] = 6
 * edgeTo[4] = 5
 * edgeTo[5] = 1
 * edgeTo[6] = 0
 * 
 * distTo[0] = 0
 * distTo[1] = 1.39
 * distTo[2] = 
 * distTo[3] = 1.10
 * distTo[4] = 2.06
 * distTo[5] = 1.71
 * distTo[6] = 0.58
 */

public class _4_4_4 {
    
}
