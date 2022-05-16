#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.3

// Show the contents of the id[] array and the number of times the array is accessed for each input pair 
// when you use weighted quick-union for the sequence 9-0 3-4 5-8 7-2 2-1 5-7 0-3 4-2. 

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
        self.array_access = self.array_access + 1;
        let mut temp = p;
        while temp != self.id[temp] {
            temp = self.id[temp];
            self.array_access = self.array_access + 2;
        }
        temp
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
        // if uf.connected(element.0, element.1) {continue};
        uf.union(element.0, element.1);
        println!("{} --- {}", element.0, element.1);
        // println!("ARRAY ACCESS - {:?}", uf.array_access);
        println!("ID - {:?}", uf.id);
        println!("SZ - {:?}", uf.sz);
    }

    println!("{:?} components", uf.count());
    println!("{:?}", uf.id);
    println!("{:?}", uf.array_access);
}

// id = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
// sz = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]

// (9,0)
// 2 x find() + 1 = 3
// id = [9, 1, 2, 3, 4, 5, 6, 7, 8, 9]
// sz = [1, 1, 1, 1, 1, 1, 1, 1, 1, 2]

// (3,4)
// 2 x find() + 1 = 3
// id = [9, 1, 2, 3, 3, 5, 6, 7, 8, 9]
// sz = [1, 1, 1, 2, 1, 1, 1, 1, 1, 2]

// (5,8)
// 2 x find() + 1 = 3
// id = [9, 1, 2, 3, 3, 5, 6, 7, 5, 9]
// sz = [1, 1, 1, 2, 1, 2, 1, 1, 1, 2]

// (7,2)
// 2 x find() + 1 = 3
// id = [9, 1, 7, 3, 3, 5, 6, 7, 5, 9]
// sz = [1, 1, 1, 2, 1, 2, 1, 2, 1, 2]

// (2,1)
// 3 + 1 + 1 = 5
// id = [9, 7, 7, 3, 3, 5, 6, 7, 5, 9]
// sz = [1, 1, 1, 2, 1, 2, 1, 3, 1, 2]


// (5,7)
// 2 x find() + 1 = 3
// id = [9, 7, 7, 3, 3, 7, 6, 7, 5, 9]
// sz = [1, 1, 1, 2, 1, 2, 1, 5, 1, 2]

// (0,3)
// 5 array access
// id = [9, 7, 7, 9, 3, 7, 6, 7, 5, 9]
// sz = [1, 1, 1, 2, 1, 2, 1, 5, 1, 4]

// (4,2)
// 9 array access
// id = [9, 7, 7, 9, 3, 7, 6, 7, 5, 7]
// sz = [1, 1, 1, 2, 1, 2, 1, 9, 1, 4]