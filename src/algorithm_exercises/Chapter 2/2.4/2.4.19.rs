#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p330 Exercises

// 2.4.19

// Implement the constructor for MaxPQ that takes an array of items as argument, 
// using the bottom-up heap construction method described on page 323 in the text.

pub struct MaxPQ<T> {
    heap: Vec<Option<T>>,
    size: usize
}

impl<T: Copy + Ord + std::fmt::Debug> MaxPQ<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new(vec: Vec<T>) -> Self {
        let length = vec.len();
        let mut new_heap = Vec::new();
        new_heap.push(None);
        for elem in vec { new_heap.push(Some(elem)) }
        let mut new_pq = MaxPQ { size: length, heap: new_heap };

        // Create heap through inductive method
        let mut k = length / 2;
        
        while k >= 1 {
            new_pq.sink(k);
            k -= 1;
        }

        new_pq
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
        while 2*index <= self.size {
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
    let mut pq = MaxPQ::new(vec);

    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
    println!("{:?}", pq.delMax().unwrap())   ;
}
