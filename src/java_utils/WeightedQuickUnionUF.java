package java_utils;

public class WeightedQuickUnionUF {
    private int[] id; // Parent link (site-indexed)
    private int[] sz; // Subtree size, using site-index as root
    private int count; // # of components

    public WeightedQuickUnionUF(int N) {
       count = N;
       id = new int[N];
       for (int i = 0; i < N; i++) id[i] = i;
       sz = new int[N];
       for (int i = 0; i < N; i++) sz[i] = 1;
    }

    public int count() {return count;}

    public boolean connected(int p, int q) {return find(p) == find(q);}

    private int find(int p) {
        // Traverse up tree
        while (p != id[p]) p = id[p];
        return p; 
    }

    public int treeSize(int p) {
        int root = find(p);
        return sz[root];
    }

    public void union(int p, int q) {
        // Find root for each node
        int i = find(p);
        int j = find(q);
        if (i == j) return;

       // Smaller tree will become a subtree of the bigger tree
        if (sz[i] < sz[j]) { 
            id[i] = j; 
            sz[j] += sz[i]; 
        } else {
            id[j] = i; 
            sz[i] += sz[j]; 
        }

        count--; 
    }
}
