package java_utils;

/**
 * Created by Rene Argento on 09/11/17.
 */
public interface EdgeWeightedGraphInterface {

    int vertices();
    int edgesCount();
    void addEdge(Edge edge);
    Iterable<Edge> adjacent(int vertex);
    Iterable<Edge> edges();

}
