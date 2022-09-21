// Arithmetic expressions. 

// Write a class that evaluates DAGs that represent arithmetic expressions. Use a vertex-indexed array to hold values corresponding to each vertex. Assume that values corresponding to leaves have been established. Describe a family of arithmetic expressions with the property that the size of the expression tree is exponentially larger than the size of the corresponding DAG (so the running time of your program for the DAG is proportional to the logarithm of the running time for the tree).

// DAG that represent artihmetic expression
// Vertex-indexed array hold values corresponding to each vertex

// Vertex = value or operator
// DFS start with an operator as the source
// If DFS reach value, return from DFS
// There is never an edge from value to value
// If DFS reach value, that is end of the DFS call and return the value.
// Division - 1 * dfs for first call, / dfs for 2nd call.

/**
 * Digraph 1:
 * 2 + (3 * 4) + (3 * 4) / 5
 *
 *      +
 *     / \
 *    v  v
 *   +   /(DIV)
 *   /\  / \
 *  v v  v v
 *  2  *   5
 *     /\
 *    v v
 *    3 4

+
 +
  2 => return (0 + 2 = 2) at 2nd DFS 
  *
   3 => return (1 * 3 = 3) at 3rd DFS
   4 => return (3 * 4 = 12) at 3rd DFS
  => return (2 + 12 = 14) at 2nd DFS
 => return 14 at 1st DFS

 => Return (12 / 5 = 2.4)
14 + 2.4 => 16.4

 */

/**
 * Digraph 2:
 * (3 * 4) + (3 * 4) + (3 * 4) + (3 * 4) / 6
 *
 *         /(DIV)
 *       /   \
 *      v     v
 *      +     6
 *    /\ / \
 *   v v v v
 *     *
 *     /\
 *    v v
 *    3 4

// If need replicated evaluation, use parallel edge

 */


public class _4_2_29 {
    
}
