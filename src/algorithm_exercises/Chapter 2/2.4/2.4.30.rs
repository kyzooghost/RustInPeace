#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod utils {
    pub mod PriorityQueue;
}

use utils::PriorityQueue::MaxPQ as MaxPQ;
use utils::PriorityQueue::MinPQ as MinPQ;

// p332 Exercises

// 2.4.30 Dynamic median-finding. 
// Design a data type that supports insert in logarithmic time, 
// find the median in constant time, 
// and delete the median in logarithmic time. 
// Hint: Use a min-heap and a max-heap.

// Hmm how can you do this with a min-heap and a max-heap?
// Insert is O(lg N) time if you use both
// But find median in constant time?
// Min heap lets you find min in O(1), max heap lets you find max in O(1)

// Ahh ok, if you maintain bigger numbers in minPQ, smaller numbers in maxPQ

pub struct MedianPQ<usize> {
    min_pq: MinPQ<usize>,
    max_pq: MaxPQ<usize>,
    size: usize
}

impl MedianPQ<usize> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        MedianPQ { size: 0, min_pq: MinPQ::new(), max_pq: MaxPQ::new() }
    }

    // Invariants to maintain
    // max of maxPQ < min of minPQ
    // maxPQ.size() - minPQ.size() <= 1

    pub fn insert(&mut self, element: usize) {
        // Start by inserting into min_pq
        if self.size == 0 {
            self.min_pq.insert(element);

        } else if self.size == 1 {
            // If element <= min of min_pq, happy case and add to max_pq
            if element <= self.min_pq.min().unwrap() {self.max_pq.insert(element)}
            // Else need to transfer min_pq element to max_pq
            // And add to min_pq
            else {
                self.max_pq.insert(self.min_pq.delMin().unwrap());
                self.min_pq.insert(element);
            }

        // Even, need to add to min_pq
        } else if self.size % 2 == 0 {
            let median_floor = self.max_pq.max().unwrap();

            // Happy case, single step add to min_pq
            if element >= median_floor {self.min_pq.insert(element)}
            // Else need to transfer max_pq to min_pq, and add element to max_pq
            else {
                self.min_pq.insert(self.max_pq.delMax().unwrap());
                self.max_pq.insert(element);
            }

        // Odd, need to add to max_pq
        } else {
            let median_ceiling = self.min_pq.min().unwrap();

            // Happy case, single step add to max_pq
            if element <= median_ceiling {self.max_pq.insert(element)}
            else {
                self.max_pq.insert(self.min_pq.delMin().unwrap());
                self.min_pq.insert(element);
            }
        }

        self.size += 1;
    }

    // Need to cast to f32 for decimals
    pub fn median(&self) -> Option<f32> {
        if self.size == 0 {return None}
    
        // If even, median is halfway between min of min_pq, and max of max_pq
        if self.size % 2 == 0 {
            return Some( ( self.max_pq.max().unwrap() as f32 + self.min_pq.min().unwrap() as f32 ) / 2.0 )
       
        // Else if odd, median is min of min_pq (is the median integer, because we add to min_pq first arbitrarily)
        } else {
            return Some (self.min_pq.min().unwrap() as f32)
        }
    }

    pub fn delete_median(&mut self) -> Option<usize> {
        if self.size == 0 {return None}

        if self.size % 2 == 1 {return self.min_pq.delMin()}
        else {return self.max_pq.delMax()}
    }
}

fn main() {
    let vec = vec![102, 56, 34, 99, 89, 101, 10, 54]; 
    let mut pq = MedianPQ::new();
    for element in vec {pq.insert(element)}   
    println!("{:?}", pq.median());
}