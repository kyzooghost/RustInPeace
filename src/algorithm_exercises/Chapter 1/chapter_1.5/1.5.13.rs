#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.13

// Weighted quick-union with path compression. 

// Modify weighted quick-union (Algorithm 1.5) to implement path compression, 
// as described in Exercise 1.5.12. 

// Give a sequence of input pairs that causes this method to produce a tree of height 4.
// Note : The amortized cost per operation for this algorithm is known to be 
// bounded by a function known as the inverse Ackermann function 
// and is less than 5 for any conceivable practical value of N.

struct UF {
    count: usize,
    id: Vec<usize>,
    array_access: usize,
    sz: Vec<usize>
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
            sz: sz_vec
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
    let mut uf = UF::new(19);

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