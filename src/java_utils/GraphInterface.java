package java_utils;

public interface GraphInterface {

    int vertices();
    int edges();
    Iterable<Integer> adjacent(int vertex);

}
