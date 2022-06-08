#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p331 Exercises

// 2.4.26

// Sink and swim without swap()

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

    // pub fn min(&self) -> Option<T> {
    //     if self.size == 0 {return None}
    //     self.heap[self.size]
    // }

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
        if self.size >= 2 {self.sink(1)};
        max
    }

    fn swim(&mut self, mut index: usize) {
        let temp = self.heap[index]; 

        while index > 1 && self.heap[index / 2] < self.heap[index] {
            self.heap[index] = self.heap[index / 2];
            index = index / 2;
        }

        self.heap[index] = temp;
    }

    fn sink(&mut self, mut index: usize) {
        let temp = self.heap[index];

        while 2*index <= self.size {
            let mut j = 2 * index;
            if j < self.size && self.heap[j] < self.heap[j + 1] {j += 1}
            if temp >= self.heap[j] {
                break
            }
            self.heap[index] = self.heap[j];
            index = j;
        }

        self.heap[index] = temp;
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