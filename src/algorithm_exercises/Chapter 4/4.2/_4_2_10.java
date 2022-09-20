// 597
// Given a DAG, does there exist a topological order that cannot result from applying a DFS-based algorithm, no matter in what order the vertices adjacent to each vertex are chosen? Prove your answer.

/*
 Proposition F, reverse postorder in a DAG is a topological sort

 Consider any edge v->w

 When dfs(v) is called, either:
 i.) dfs(w) has already been called and has returned (w is marked)
 ii.) dfs(w) has not yet been called (w is unmarked), so in the future dfs(w) will be called and return before dfs(v) returns
 iii.) dfs(w) has been called and has not yet returned when dfs(v) is called. Impossible in DAG, because this implies w -> v path and hence a cycle.

 In either of the two possible cases, dfs(w) returns before dfs(v), so w appears before v in postorder, and after v in reverse postorder.

 Topological order is reverse postorder of a digraph. Reverse postorder is from DFS. Hence topological order can always be found from DFS.

 If topological order cannot result from DFS, it means not a DAG and is a directed cycle, which means there cannot be a topological order.
 */


public class _4_2_10 {
    
}
