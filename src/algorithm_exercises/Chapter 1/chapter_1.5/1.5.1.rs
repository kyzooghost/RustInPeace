#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.5.1

// Show the contents of the id[] array and the number of times the array is accessed for each input pair 
// when you use quick-find for the sequence 9-0 3-4 5-8 7-2 2-1 5-7 0-3 4-2.

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
        println!("Q");
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

// id = [1, 1, 1, 1, 1, 1, 6, 1, 1, 1]
// 126 array accesses
// For each tuple - 4 for 4x find(), N for looping through id[] array, 1 to N-1 for updating nodes
// => N(2N + 3) worst case, N(N + 5) best case