package java_utils;

// I'm a fan of this class, because if allows us to store a path efficiently, and compare paths efficiently
public class DirectedEdgePath implements Comparable<DirectedEdgePath> {
    private int lastVertexInPath;
    private double weight;
    private int length;
    private DirectedEdge lastEdgeInPath;
    // Reference to previous Path (Store as reference for O(1) constructor function, otherwise storing as collection and requiring deep clone creates an O(N) constructor function).
    private DirectedEdgePath previousPath;

    // Create completely new path starting from single source
    public DirectedEdgePath(int source) {
        this.lastVertexInPath = source;
        this.weight = 0.0;
        this.length = 1;
        // lastEdgeInPath and previousPath initialised as `null`.
    }

    // Construct new Path object, by adding an edge to a previous Path.
    public DirectedEdgePath(DirectedEdgePath previousPath, DirectedEdge directedEdge) {
        this.lastVertexInPath = directedEdge.to();
        this.previousPath = previousPath;
        this.lastEdgeInPath = directedEdge;
        this.weight = previousPath.weight() + directedEdge.weight();
        this.length = previousPath.length() + 1;
    }

    public double weight() {return this.weight;}
    public int length() {return this.length;}
    public double directedEdge() {return this.directedEdge();}

    @Override
    public int compareTo(DirectedEdgePath other) {
        if (this.weight < other.weight) {
            return -1;
        } else if (this.weight > other.weight) {
            return 1;
        } else {
            return 0;
        }
    }

    public Iterable<DirectedEdge> getPath() {
        Stack<DirectedEdge> path = new Stack<DirectedEdge>();
        path.push(this.lastEdgeInPath);

        DirectedEdgePath iterator = previousPath;

        while (iterator != null && iterator.lastEdgeInPath != null) {
            path.push(iterator.lastEdgeInPath);
            iterator = iterator.previousPath;
        }

        return path;
    }
}
