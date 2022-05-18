#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.14

// Develop a UF implementation that uses the same basic strategy 
// as weighted quick-union but keeps track of tree height 
// and always links the shorter tree to the taller one. 

// Prove a logarithmic upper bound on the height of the trees for N sites with your algorithm.


// Tree height will only increase when uniting between two trees of same height
// In worst case, two trees of same height will be united lg N times
// So height of tree will increase less than lg N times

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

            println!("{:?}", self.id);
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
    println!("{:?}", uf.max_height);
    println!("{:?}", uf.array_access);
}