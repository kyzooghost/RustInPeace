package java_utils;

public class DepthFirstOrder {
    private boolean[] marked;
    private Queue<Integer> pre; // vertices in preorder (order of DFS call made)
    private Queue<Integer> post; // vertices in postorder (order of DFS call completed)
    private Stack<Integer> reversePost; // vertices in reverse postorder (also topological sort)

    public DepthFirstOrder(Digraph G) {
        pre = new Queue<Integer>();
        post = new Queue<Integer>();
        reversePost = new Stack<Integer>();
        marked = new boolean[G.vertices()];

        for (int v = 0; v < G.vertices(); v++) {
            if (!marked[v]) dfs(G, v);
        }
    }

    private void dfs(Digraph G, int v) {
        pre.enqueue(v);
        for (int w: G.adjacent(v)) {
            if (!marked[w]) dfs(G, w);
        }
        post.enqueue(v);
        reversePost.push(v);
    }

    public Iterable<Integer> pre() {return pre;}
    public Iterable<Integer> post() {return post;}
    public Iterable<Integer> reversePost() {return reversePost;}
}