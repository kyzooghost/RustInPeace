#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.20

// Dynamic growth. 

// Using linked lists or a resizing array, 
// develop a weighted quick-union implementation that removes the restriction 
// on needing the number of objects ahead of time. 

// Add a method newSite() to the API, which returns an int identifier.

struct UF {
    count: usize,
    id: Vec<usize>,
    array_access: usize,
    sz: Vec<usize>,
    num_of_nodes: usize
}

impl UF {
    pub fn new() -> Self {
        let mut id_vec = Vec::new();
        let mut sz_vec = Vec::new();

        UF {
            count: 0,
            id: id_vec,
            array_access: 0,
            sz: sz_vec,
            num_of_nodes: 0
        }
    }

    pub fn newNode(&mut self) -> usize {
        self.id.push(self.num_of_nodes);
        self.sz.push(1);
        self.count = self.count + 1;
        self.num_of_nodes = self.num_of_nodes + 1;
        self.num_of_nodes
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn numOfNodes(&self) -> usize {
        self.num_of_nodes
    }

    pub fn find(&mut self, p: usize) -> usize {
        self.array_access = self.array_access + 1;
        let mut root = p;
        while root != self.id[root] {
            root = self.id[root];
            self.array_access = self.array_access + 2;
        }
        root
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        if p >= self.num_of_nodes || q >= self.num_of_nodes {panic!("site does not exist")}
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        if p >= self.num_of_nodes || q >= self.num_of_nodes {panic!("site does not exist")}
        let pRoot = self.find(p);
        let qRoot = self.find(q);

        if pRoot == qRoot {}
        else {
            if self.sz[pRoot] < self.sz[qRoot] {
                self.id[pRoot] = qRoot;
                self.sz[qRoot] = self.sz[qRoot] + self.sz[pRoot]
            } else {
                self.id[qRoot] = pRoot;
                self.sz[pRoot] = self.sz[pRoot] + self.sz[qRoot]  
            }
            self.array_access = self.array_access + 1; // Excluding 3 array access to self.size[] 
            self.count = self.count - 1;
        }
    }
}

fn main() {
    let mut uf = UF::new();

    for _ in 0..20 {
        println!("{:?}", uf.newNode());
    }

    let connections = vec![
        (0, 1),
        (0, 2),
        (0, 3),
        (6, 7),
        (8, 9),
        (6, 8),
        (0, 6),
        (10, 11),
        (10, 12),
        (10, 13),
        (10, 14),
        (10, 15),
        (10, 16),
        (10, 17),
        (10, 18),
        (0, 10)
    ];

    for element in connections {
        if uf.connected(element.0, element.1) {continue};
        uf.union(element.0, element.1);
        println!("{} --- {}", element.0, element.1);
    }

    println!("{:?} components", uf.count());
    println!("{:?}", uf.id);
    println!("{:?}", uf.array_access);
}