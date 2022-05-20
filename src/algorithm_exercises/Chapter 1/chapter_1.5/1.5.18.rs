#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.17

// Random grid generator. 

// Write a program RandomGrid that takes an int value N from the command line, 
// generates all the connections in an N-by-N grid, 
// puts them in random order,
// randomly orients them(so that p qand q p are equally likely to occur), 
// and prints the result to standard output. 

// To randomly order the connections, use a RandomBag
// To encapsulate p and q in a single object,
// use the Connection nested class shown below. 

// Package your program as two static methods: generate(), 
// which takes N as argument and returns an array of connections, 

// and main(), which takes N from the command line, calls generate(), 
// and iterates through the returned array to print the connections.

// generate() - generate all connections in N-by-N grid
// Put them in random order and randomly orient them


struct UF {
    count: usize,
    id: Vec<usize>,
    array_access: usize,
    sz: Vec<usize>,
    max_height: usize
}

impl UF {
    pub fn new(num_nodes: usize) -> Self {
        let mut id_vec = Vec::new();
        let mut sz_vec = Vec::new();

        for i in 0..num_nodes {
            id_vec.push(i);
            sz_vec.push(1);
        }

        UF {
            count: num_nodes,
            id: id_vec,
            array_access: 0,
            sz: sz_vec,
            max_height: 0
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn find(&mut self, p: usize) -> usize {
        
        let mut root = p;
        let mut temp = p;

        self.array_access = self.array_access + 1;        
        while root != self.id[root] {
            root = self.id[root];
            self.array_access = self.array_access + 2;
        }

        while temp != self.id[temp] {
            let next_parent = self.id[temp];
            self.id[temp] = root;
            temp = next_parent;
        }

        root
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pRoot = self.find(p);
        let qRoot = self.find(q);

        if pRoot == qRoot {}
        else {
            if self.sz[pRoot] < self.sz[qRoot] {
                self.id[pRoot] = qRoot;
            } else if self.sz[qRoot] < self.sz[pRoot] {
                self.id[qRoot] = pRoot;
            } else {
                self.id[pRoot] = qRoot;
                self.sz[qRoot] = self.sz[qRoot] + 1;
                if self.sz[qRoot] > self.max_height {self.max_height = self.sz[qRoot]}
            }

            // println!("{:?}", self.id);
            self.array_access = self.array_access + 1; // Excluding 3 array access to self.size[] 
            self.count = self.count - 1;
        }
    }
}

fn main() {
    // let mut uf = UF::new(19);

    // let connections = vec![
    //     (0, 1),
    //     (0, 2),
    //     (0, 3),
    //     (6, 7),
    //     (8, 9),
    //     (6, 8),
    //     (0, 6),
    //     (10, 11),
    //     (10, 12),
    //     (10, 13),
    //     (10, 14),
    //     (10, 15),
    //     (10, 16),
    //     (10, 17),
    //     (10, 18),
    //     (0, 10)
    // ];

    // for element in connections {
    //     if uf.connected(element.0, element.1) {continue};
    //     uf.union(element.0, element.1);
    //     println!("{} --- {}", element.0, element.1);
    // }

    // println!("{:?} components", uf.count());
    // println!("{:?}", uf.id);
    // println!("{:?}", uf.max_height);
    // println!("{:?}", uf.array_access);

    println!("{:?}", generate(20));
}

fn generate(N: usize) -> usize {
    let num_nodes = N*N;
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();

    let mut connections = 0;
    let mut uf = UF::new(num_nodes);

    while uf.count() > 1 {
        // We are already randomly generating the node IDs, will skip the implementation details of
        // randomly ordering the connections and randomly orientating them
        let x = rng.gen_range(0..num_nodes);
        let y = rng.gen_range(0..num_nodes);
        
        if x != y {
            connections = connections + 1;
            if !uf.connected(x, y) {
                uf.union(x, y);
                println!("{} --- {}", x, y);
            }
        }
    }

    connections
}