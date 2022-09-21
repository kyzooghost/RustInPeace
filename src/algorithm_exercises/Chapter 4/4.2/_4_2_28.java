// Give a formula for the number of V-vertex DAGs with E edges.
// Difference between digraph and DAGs? 

/*
 
Maximum # of edges a vertex V1 can have without generating cycle is V - 1
Then V2 can have (V - 1) - 1.,,
So max # of edges is (V - 1) + (V - 2) + ... + 1 + 0 = (V - 1) * V / 2 => Arithmetic sum

So ((V - 1) * V / 2) choose E

 */

public class _4_2_28 {
    
}
