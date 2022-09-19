// Design and implement an API EuclideanGraph for graphs whose vertices are points in the plane that include coordinates. Include a method show() that uses StdDraw to draw the graph.

import java_utils.Bag;
import java_utils.SeparateChainingHashTable;
import java.util.ArrayList;

@SuppressWarnings("unchecked")
public class _4_1_37 {

    public class Point {
        private final int x;
        private final int y;

        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        public int x() {return this.x;}
        public int y() {return this.y;}
    }

    public class EuclideanGraph {
        
        private int vertices;
        protected int edges;
        private ArrayList<Bag<Integer>> adjacent;
        private SeparateChainingHashTable<Point, Integer> pointToIDMap;
        private ArrayList<Point> IDtoPointMap;

        public EuclideanGraph() {
            this.vertices = 0;
            this.edges = 0;
            this.adjacent = new ArrayList<Bag<Integer>>();
            this.pointToIDMap = new SeparateChainingHashTable<Point, Integer>();
            this.IDtoPointMap = new ArrayList<Point>();
        }
    
        public int vertices() {return vertices;}
        public int edges() {return edges;}
        public ArrayList<Bag<Integer>> getAdjacencyList() {return adjacent;}
    
        // Support parallel edges, because Bag doesn't have 'contains' method.
        public void addEdge(Point point1, Point point2) {
            if (!pointToIDMap.contains(point1)) {
                pointToIDMap.put(point1, vertices);
                IDtoPointMap.add(point1);
                Bag<Integer> new_bag = new Bag<Integer>();
                adjacent.add(new_bag);
                this.vertices += 1;
            }

            if (!pointToIDMap.contains(point2)) {
                pointToIDMap.put(point2, vertices);
                IDtoPointMap.add(point2);
                Bag<Integer> new_bag = new Bag<Integer>();
                adjacent.add(new_bag);
                this.vertices += 1;
            }

            int id1 = pointToIDMap.get(point1);
            int id2 = pointToIDMap.get(point2);
            Bag<Integer> bag1 = adjacent.get(id1);          
            Bag<Integer> bag2 = adjacent.get(id2);
            bag1.add(id2);
            bag2.add(id1);
            adjacent.set(id1, bag1);
            adjacent.set(id2, bag2);
            edges++;
        }
    
        public int degree(Point point) {
            if (!pointToIDMap.contains(point)) {return 0;}
            int id = pointToIDMap.get(point);
            return adjacent.get(id).size();
        }

        public Iterable<Point> adjacent(Point point) {
            Bag<Point> point_bag = new Bag<Point>();
            if (!pointToIDMap.contains(point)) {return point_bag;}
            int id = pointToIDMap.get(point);
            Bag<Integer> id_bag = adjacent.get(id);

            for (int i : id_bag) {
                point_bag.add(IDtoPointMap.get(i));
            }

            return point_bag;
        }
    }
    public static void main(String[] args) {
        _4_1_37 exercise = new _4_1_37();
        EuclideanGraph graph = exercise.new EuclideanGraph();

        Point point1 = exercise.new Point(1, 2);
        Point point2 = exercise.new Point(3, 4);
        Point point3 = exercise.new Point(5, 6);

        graph.addEdge(point1, point2);
        graph.addEdge(point1, point3);

        for (Point point : graph.adjacent(point1)) {
            System.out.println(point.x + "," + point.y);
        }

        System.out.println("------");

        for (Point point : graph.adjacent(point2)) {
            System.out.println(point.x + "," + point.y);
        }

        System.out.println("------");

        for (Point point : graph.adjacent(point3)) {
            System.out.println(point.x + "," + point.y);
        }
    }
}
