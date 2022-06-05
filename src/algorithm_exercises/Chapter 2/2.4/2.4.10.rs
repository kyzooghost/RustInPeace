#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p329 Exercises

// 2.4.10

// Suppose that we wish to avoid wasting one position in a heap-ordered array pq[], 
// putting the largest value in pq[0], its children in pq[1] and pq[2], and so forth, 
// proceeding in level order. Where are the parents and children of pq[k]?

// 0
// 1 2
// 3 4 5 6
// 7 8 9 10 11 12 13 14

// children of pq[k] in pq[2 * (k + 1)] & pq[2 * (k + 1) - 1]
// parent in pq[k] in pq[(k - 1) / 2]

pub struct MaxPQ<T> {
    heap: Vec<Option<T>>,
    size: usize
}

impl<T: Copy + Ord + std::fmt::Debug> MaxPQ<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new() -> Self {
        MaxPQ { size: 0, heap: vec![None] }
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn max(&self) -> Option<T> {
        if self.size == 0 {return None}
        self.heap[1]
    }

    pub fn insert(&mut self, element: T) {
        self.size += 1;
        self.heap.push(Some(element));
        self.swim(self.size);
    }

    pub fn delMax(&mut self) -> Option<T> {
        if self.size == 0 {return None}

        self.heap.swap(1, self.size);
        let max = self.heap.pop().unwrap();
        self.size -= 1;
        self.sink(1);
        max
    }

    fn swim(&mut self, mut index: usize) {
        while index > 1 && self.heap[index / 2] < self.heap[index] {
            self.heap.swap(index, index / 2);
            index = index / 2;
        }
    }

    fn sink(&mut self, mut index: usize) {
        while 2*index < self.size {
            let mut j = 2 * index;
            if j < self.size && self.heap[j] < self.heap[j + 1] {j += 1}
            if self.heap[index] >= self.heap[j] {break}
            self.heap.swap(index, j);
            index = j;
        }   
    }
}

fn main() {
    let vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    
    let mut pq = MaxPQ::new();

    for element in vec {pq.insert(element);}
    println!("{:?}", pq.heap)   ;
}
