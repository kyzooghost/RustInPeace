/*
 * Write a CPM (Critical path method) client that prints all critical paths.
 */

import java_utils.Stack;
import java_utils.Bag;
import java_utils.DirectedEdge;
import java_utils.EdgeWeightedDigraph;
import java_utils.AcyclicLP;
import java.util.Arrays;

public class _4_4_18 {
    
    public class Path implements Comparable<Path> {
        private int lastVertexInPath;
        private double weight;
        private DirectedEdge lastEdgeInPath;
        // Reference to previous Path (Store as reference for O(1) constructor function, otherwise storing as collection and requiring deep clone creates an O(N) constructor function).
        private Path previousPath;

        // Create completely new path starting from single source
        Path(int source) {
            this.lastVertexInPath = source;
            this.weight = 0.0;
            // lastEdgeInPath and previousPath initialised as `null`.
        }

        // Construct new Path object, by adding an edge to a previous Path.
        Path(Path previousPath, DirectedEdge directedEdge) {
            this.lastVertexInPath = directedEdge.to();
            this.previousPath = previousPath;
            this.lastEdgeInPath = directedEdge;
            this.weight = previousPath.weight() + directedEdge.weight();
        }

        public double weight() {return this.weight;}
        public double directedEdge() {return this.directedEdge();}

        @Override
        public int compareTo(Path other) {
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

            Path iterator = previousPath;

            while (iterator != null && iterator.lastEdgeInPath != null) {
                path.push(iterator.lastEdgeInPath);
                iterator = iterator.previousPath;
            }

            return path;
        }
    }

    public class Job {
        private int jobID;
        private int duration;

        // Collection of jobIDs that that jobID must precede
        private Bag<Integer> precedenceConstraints;

        Job (int jobID, int duration, Iterable<Integer> proceedingJobIDs) {
            this.jobID = jobID;
            this.duration = duration;
            this.precedenceConstraints = new Bag<Integer>();
            for (int i : proceedingJobIDs) {
                precedenceConstraints.add(i);
            }
        }

        public int jobID() {return this.jobID;}
        public int duration() {return this.duration;}
        public Iterable<Integer> precedenceConstraints() {return this.precedenceConstraints;}
    }

    public class CriticalPath {
        private AcyclicLP criticalPath;
        private EdgeWeightedDigraph graph;
        private Bag<Path> criticalPaths;
        private int source;
        private int target;
        private int num_jobs;

        CriticalPath (Iterable<Job> jobs) {
            int num_jobs = 0;
            for (Job job : jobs) {num_jobs += 1;}
            this.num_jobs = num_jobs;
            this.graph = new EdgeWeightedDigraph(2 * num_jobs + 2);
            int source = 2 * num_jobs;
            this.source = source;
            int target = 2 * num_jobs + 1;
            this.target = target;

            for (Job job : jobs) {
                graph.addEdge(new DirectedEdge(job.jobID(), job.jobID() + num_jobs, job.duration()));
                graph.addEdge(new DirectedEdge(source, job.jobID(), 0.0));
                graph.addEdge(new DirectedEdge(job.jobID() + num_jobs, target, 0.0));

                for (int proceedingJobID : job.precedenceConstraints()) {
                    graph.addEdge(new DirectedEdge(job.jobID() + num_jobs, proceedingJobID, 0.0));
                }
            }

            this.criticalPath = new AcyclicLP(graph, source);
            findCriticalPaths();
        }

        private void findCriticalPaths() {
            this.criticalPaths = new Bag<Path>();
            Bag<Integer> startingJobs = new Bag<Integer>();
            int num_jobs = (graph.vertices() - 2) / 2;
            int source = graph.vertices() - 2;
            int target = graph.vertices() - 1;
            double criticalPathDistance = this.criticalPath.distTo(target);

            for (int i = 0; i < num_jobs; i++) {
                if (this.criticalPath.distTo(i) == 0) {startingJobs.add(i);}
            }

            // All critical path must start from jobID with distTo[] == 0
            for (int i: startingJobs) {
                double currentPathWeight = 0;

                Path emptyPath = new Path(this.source);
                visit(
                    0.0, 
                    new Path(emptyPath, new DirectedEdge(source, i, 0.0)),
                    i
                );

                System.out.println(i);
            }

            return;
        }

        private void visit(double currentWeight, Path currentPath, int vertex) {
            DirectedEdge edge = currentPath.lastEdgeInPath;

            // Base case: found target
            if (edge.to() == this.target) {
                // Found a critical path
                if (currentWeight == this.criticalPath.distTo(target)) {
                    this.criticalPaths.add(currentPath);
                }
                return;
            }

            for (DirectedEdge e : this.graph.adjacent(edge.to())) {
                visit(currentWeight + edge.weight(), new Path(currentPath, e), e.to());
            }

        }

        public void printStartTimes() {
            int num_jobs = (graph.vertices() - 2) / 2;

            System.out.println("Start times:");
            for (int i = 0; i < num_jobs; i++) {
                System.out.println("job " + i + ": " + criticalPath.distTo(i));
            }
            System.out.println("\nFinish time: " + criticalPath.distTo(2 * num_jobs + 1));
        }

        public void printLongestPath() {
            int source = graph.vertices() - 2;
            int target = graph.vertices() - 1;
            Stack<Integer> path = new Stack<Integer>();
            path.push(Integer.MIN_VALUE);

            for (DirectedEdge e: criticalPath.pathTo(target)) {
                int to = parseNode(e.to());
                
                if (path.peek() != to) {
                    path.push(to);
                }
            }

            Stack<Integer> printedPath = new Stack<Integer>();

            while (!path.isEmpty()) {
                printedPath.push(path.pop());
            }

            StringBuilder string = new StringBuilder();

            for (int i : printedPath) {
                if (i == Integer.MIN_VALUE) {string.append("source->");}
                else if (i == Integer.MAX_VALUE) {string.append("target");}
                else {string.append(String.valueOf(i)).append("->");}
            }
            
            System.out.println(string);
        }

        private int parseNode(int vertex) {
            int target = graph.vertices() - 1;
            int source = graph.vertices() - 2;
            int num_jobs = (graph.vertices() - 2) / 2;

            if (vertex == target) {return Integer.MAX_VALUE;}
            else if (vertex == source) {return Integer.MIN_VALUE;}
            else if (vertex < num_jobs) {
                return vertex;
            } else {
                return (vertex - num_jobs);
            }
        }

        private void printCriticalPaths() {
            for (Path path : this.criticalPaths) {
                Stack<Integer> pathStack = new Stack<Integer>();
                pathStack.push(Integer.MIN_VALUE);

                // Iterate from target to source
                // Create stack, with source at the top
                for (DirectedEdge edge : path.getPath()) {
                    int to = parseNode(edge.to());
                    if (pathStack.peek() != to) {
                        pathStack.push(to);
                    }
                }

                Stack<Integer> printedPath = new Stack<Integer>();
                while (!pathStack.isEmpty()) {printedPath.push(pathStack.pop());}

                StringBuilder string = new StringBuilder();

                for (int i : printedPath) {
                    if (i == Integer.MIN_VALUE) {string.append("source->");}
                    else if (i == Integer.MAX_VALUE) {string.append("target");}
                    else {string.append(String.valueOf(i)).append("->");}
                }
                
                System.out.println(string);
            }
        }
    }

    public static void main(String[] args) {
        _4_4_18 exercise = new _4_4_18();

        {
            Job job0 = exercise.new Job(0, 41, Arrays.asList(new Integer[]{1, 7, 9}));
            Job job1 = exercise.new Job(1, 51, Arrays.asList(new Integer[]{2}));
            Job job2 = exercise.new Job(2, 50, Arrays.asList(new Integer[]{}));
            Job job3 = exercise.new Job(3, 36, Arrays.asList(new Integer[]{}));
            Job job4 = exercise.new Job(4, 38, Arrays.asList(new Integer[]{}));
            Job job5 = exercise.new Job(5, 45, Arrays.asList(new Integer[]{}));
            Job job6 = exercise.new Job(6, 21, Arrays.asList(new Integer[]{3, 8}));
            Job job7 = exercise.new Job(7, 32, Arrays.asList(new Integer[]{3, 8}));
            Job job8 = exercise.new Job(8, 32, Arrays.asList(new Integer[]{2}));
            Job job9 = exercise.new Job(9, 29, Arrays.asList(new Integer[]{4, 6}));

            CriticalPath path = exercise.new CriticalPath(Arrays.asList(new Job[]{
                job0, 
                job1,
                job2,
                job3,
                job4,
                job5,
                job6,
                job7,
                job8,
                job9,
            }));

            path.printStartTimes();
            System.out.println("------");
            path.printLongestPath();
            System.out.println("------");
            path.printCriticalPaths();
        }

        {
            Job job0 = exercise.new Job(0, 10, Arrays.asList(new Integer[]{1, 4}));
            Job job1 = exercise.new Job(1, 15, Arrays.asList(new Integer[]{2}));
            Job job2 = exercise.new Job(2, 5, Arrays.asList(new Integer[]{3}));
            Job job3 = exercise.new Job(3, 5, Arrays.asList(new Integer[]{}));
            Job job4 = exercise.new Job(4, 20, Arrays.asList(new Integer[]{5}));
            Job job5 = exercise.new Job(5, 5, Arrays.asList(new Integer[]{}));

            CriticalPath path = exercise.new CriticalPath(Arrays.asList(new Job[]{
                job0, 
                job1,
                job2,
                job3,
                job4,
                job5,
            }));

            path.printStartTimes();
            System.out.println("------");
            path.printLongestPath();
            System.out.println("------");
            path.printCriticalPaths();
        }
    }

}

