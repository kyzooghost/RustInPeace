#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.2

// Show the contents of the id[] array and the number of times the array is accessed for each input pair 
// when you use quick-union for the sequence 9-0 3-4 5-8 7-2 2-1 5-7 0-3 4-2. 
// In addition, draw the forest of trees represented by the id[] array after each input pair is processed.

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
            self.array_access = self.array_access + 1;
            self.id[pRoot] = qRoot;
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
        println!("ARRAY ACCESS - {:?}", uf.array_access);
        println!("ID - {:?}", uf.id);
    }

    println!("{:?} components", uf.count());
    println!("{:?}", uf.id);
    println!("{:?}", uf.array_access);
}

// id = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

// (9,0)
// id = [0, 1, 2, 3, 4, 5, 6, 7, 8, 0]
// 2 x fund() + 1x union() = 3

// (3,4)
// id = [0, 1, 2, 4, 4, 5, 6, 7, 8, 0]
// 2 x fund() + 1x union() = 3

// (5,8)
// id = [0, 1, 2, 4, 4, 8, 6, 7, 8, 0]
// 2 x fund() + 1x union() = 3

// (7,2)
// id = [0, 1, 2, 4, 4, 8, 6, 2, 8, 0]
// 2 x fund() + 1x union() = 3

// (2,1)
// id = [0, 1, 1, 4, 4, 8, 6, 2, 8, 0]
// 2 x fund() + 1x union() = 3

// (5,7)
// id = [0, 1, 1, 4, 4, 8, 6, 2, 1, 0]
// find() = 3 + 5 = 8
// 9 array access

// (0,3)
// id = [4, 1, 1, 4, 4, 8, 6, 2, 1, 0]
// find() = 1 + 3 = 4
// 5 array access

// (4,2)
// id = [4, 1, 1, 4, 1, 8, 6, 2, 1, 0]
// find () = 4 => 5 array access

// Total of 32 array accesses