#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.12

// Quick-union with path compression. 

// Modify quick-union (page 224) to include path compression, 
// by adding a loop to union() that links every site on the 
// paths from p and q to the roots of their trees to the root of the new tree. 

// Give a sequence of input pairs that causes this method to produce a path of length 4. 
// Note : The amortized cost per operation for this algorithm is known to be logarithmic.

struct UF {
    count: usize,
    id: Vec<usize>,
    array_access: usize
}

impl UF {
    pub fn new(num_nodes: usize) -> Self {
        let mut id_vec = Vec::new();

        for i in 0..num_nodes {
            id_vec.push(i)
        }

        UF {
            count: num_nodes,
            id: id_vec,
            array_access: 0
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
            self.array_access = self.array_access + 1;
            self.id[pRoot] = qRoot;
            self.count = self.count - 1;
        }

        println!("{:?}", self.id);
    }
}

fn main() {
    let mut uf = UF::new(10);

    let connections = vec![
        (9,0), 
        (3,4),
        (5,8),
        (7,2),
        (2,1),
        (5,7),
        (0,3),
        (4,2),
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