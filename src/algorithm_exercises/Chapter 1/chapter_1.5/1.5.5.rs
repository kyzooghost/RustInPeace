#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.5

// Estimate the minimum amount of time (in days) that would be required 
// for quick-find to solve a dynamic connectivity problem with 10^9 sites and 10^6 input pairs, 
// on a computer capable of executing 10^9 instructions per second. 
// Assume that each iteration of the inner for loop requires 10 machine instructions.

// Need to call union 10^6 times
// Assume find() has constant time so negligible in our analysis
// How many times does the for-loop run within each union call? Need to run over the entire array of sites
// So each union call requires 10^9 for-loop iterations => 10^10 machine instructions
// To call union() for each input pair => 10^(10+6) = 10^16 machine instructions
// => 10^7 seconds => ~115.7 days

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
        self.id[p]
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pID = self.find(p);
        let qID = self.find(q);

        if pID == qID {}
        else {
            for i in 0..self.id.len() {
                self.array_access = self.array_access + 1;
                if self.id[i] == pID {
                    self.array_access = self.array_access + 1;
                    self.id[i] = qID
                }
            }
    
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