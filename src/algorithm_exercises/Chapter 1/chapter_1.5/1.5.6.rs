#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.6

// Estimate the minimum amount of time (in days) that would be required 
// for weighted quick-union to solve a dynamic connectivity problem with 10^9 sites and 10^6 input pairs, 
// on a computer capable of executing 10^9 instructions per second. 
// Assume that each iteration of the inner for loop requires 10 machine instructions.

// Need to call union 10^6 times
// find() - assume each loop iteration = 10 machine instructions, worst case ln(10^9) iterations
// So 2x find() = 2 x 10 x lg (10^9) = 600 machine instructions
// Assume constant 10 machine instructions for the union() implementation after the 2 find() expressions
// So 610 machine instructions per union() call
// Call union() 10^6 times
// ~10^6 * 610
// ~10.8 * 6 machine instructions
// ~0.6 seconds

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
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4)
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