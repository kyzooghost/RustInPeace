#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.11

// Implement weighted quick-find, 
// where you always change the id[] entries 
// of the smaller component to the identifier of the larger component. 

// How does this change affect performance?

// Don't think it really changes performance much
// find() still O(1)
// union() still O(n)
// ? Less parent updates

struct UF {
    count: usize,
    id: Vec<usize>,
    sz: Vec<usize>,
    array_access: usize
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
            sz: sz_vec,
            array_access: 0
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn find(&mut self, p: usize) -> usize {
        self.array_access = self.array_access + 1;
        self.id[p]
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pID = self.find(p);
        let qID = self.find(q);
        if pID == qID {}

        let pSize = self.sz[pID];
        let qSize = self.sz[qID];

        if pSize < qSize {
            for i in 0..self.id.len() {
                self.array_access = self.array_access + 1;
                if self.id[i] == pID {
                    self.array_access = self.array_access + 1;
                    self.id[i] = qID;
                    self.sz[qID] = qSize + pSize;
                    self.sz[i] = qSize + pSize;
                }
            }
        } else {
            for i in 0..self.id.len() {
                self.array_access = self.array_access + 1;
                if self.id[i] == qID {
                    self.array_access = self.array_access + 1;
                    self.id[i] = pID;
                    self.sz[pID] = pSize + qSize;
                    self.sz[i] = pSize + qSize;
                }
            }
        }

        self.count = self.count - 1;
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
    println!("{:?}", uf.sz);
    println!("{:?}", uf.array_access);
}
