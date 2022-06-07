#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p331 Exercises

// 2.4.25

// Computational number theory. 
// Write a program CubeSum.java that prints out all integers of the form 
// a3 + b3 where a and b are integers between 0 and N in sorted order, 
// without using excessive space. 

// That is, instead of computing an array of the N2 sums and sorting them, 
// build a minimum-oriented priority queue, 
// initially containing (03, 0, 0), (13, 1, 0), (23, 2, 0), . . . , (N3, N, 0). 
// Then, while the priority queue is non-empty, remove the smallest item(i3 + j3, i, j), 
// print it, and then, if j < N, insert the item (i3 + (j+1)3, i, j+1). 

// Use this program to find all distinct integers a, b, c, and d between 0 and 10^6 such that a3 + b3 = c3 + d3.

// Work with tuples (i^3 + j^3, i, j)
// Use minimum-orientated priority queue
// Minimum space?
// Find all distinct integers - a, b, c, d - between 0 and 10^6, such that a^3 + b^3 = c^3 + d^3
// Initially put (..., 0, 0), (.., 1, 0), .., (.., N^3, N, 0) into the priority queue
// Remove the smallest item, then if j < N, insert the item (.., i, j + 1)
// So you're only storing N items in the priority queue at once, rather than N^2. 
// Need a structure to store your matches
// Need to modify the MinPQ to work for the tuple structure
// Also I don't trust the MaxPQ with a binary tree data structure here, it seems slower than using the MinPQ with a vector

// Yea tested, the pq with vector is 3-5x faster than the binary tree

// Not sure how I can avoid O(N^2) running time.
// It seems like you need to run through all combinations of i and j, for i <= N && j <= N
// You just avoid O(N^2) space requirements, and use O(N) space instead

pub struct MinPQ {
    heap: Vec<Option<(usize, usize, usize)>>,
    size: usize
}

impl MinPQ {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new() -> Self {
        MinPQ { size: 0, heap: vec![None] }
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn min(&self) -> Option<(usize, usize, usize)> {
        if self.size == 0 {return None}
        self.heap[1]
    }

    pub fn insert(&mut self, element: (usize, usize, usize)) {
        self.size += 1;
        self.heap.push(Some(element));
        self.swim(self.size);
    }

    pub fn delMin(&mut self) -> Option<(usize, usize, usize)> {
        if self.size == 0 {return None}

        self.heap.swap(1, self.size);
        let max = self.heap.pop().unwrap();
        self.size -= 1;
        self.sink(1);
        max
    }

    fn swim(&mut self, mut index: usize) {
        while index > 1 && self.heap[index / 2].unwrap().0 > self.heap[index].unwrap().0 {
            self.heap.swap(index, index / 2);
            index = index / 2;
        }
    }

    fn sink(&mut self, mut index: usize) {
        while 2*index <= self.size {
            let mut j = 2 * index;
            if j < self.size && self.heap[j].unwrap().0 > self.heap[j + 1].unwrap().0 {j += 1}
            if self.heap[index].unwrap().0 <= self.heap[j].unwrap().0 {break}
            self.heap.swap(index, j);
            index = j;
        }   
    }
}

fn main() {
    let mut distinct_integers:Vec<((usize, usize), (usize, usize))> = Vec::new(); // Store all distinct integer pairs
    let mut pq = MinPQ::new();
    let N = 2000 as usize;
    for i in 0..N {pq.insert((i.pow(3), i, 0))} // O(N)

    let mut removed_tuple_cache: Vec<(usize, usize, usize)> = Vec::new(); // Store all removed tuples with same cubic sum
    let mut counter = 0;

    use std::time::Instant;
    let now = Instant::now();

    // Loop until pq is empty
    while !pq.isEmpty() {
        let min = pq.delMin().unwrap(); // Remove smallest item from pq
        
        // Empty at start, just put into cache
        if removed_tuple_cache.is_empty() {
            removed_tuple_cache.push(min)
        // Else, compare smallest items to contents of removed_tuple_cache
        } else {
            // If same cubic sum, add to removed_tuple_cache
            if removed_tuple_cache[0].0 == min.0 {

                let mut distinct_pair_flag = true;

                // Check to make sure is distinct pair
                for i in 0..removed_tuple_cache.len() {
                    if min.1 == removed_tuple_cache[i].2 && min.2 == removed_tuple_cache[i].1 {distinct_pair_flag = false}
                }

                if distinct_pair_flag {removed_tuple_cache.push(min);}
                
            // Else not same cubic sum
            //      If more than one item in cache, add all possible distinct pairs into distinct_integers
            //      Empty cache
            //      Add to freshly emptied cache
            } else {
                if removed_tuple_cache.len() > 1 {
                    for i in 0..removed_tuple_cache.len() {
                        for j in i+1..removed_tuple_cache.len() {
                                    // Empty all distinct pairs into distinct_integers
                                    distinct_integers.push(
                                        (
                                            (removed_tuple_cache[i].1, removed_tuple_cache[i].2)
                                            , (removed_tuple_cache[j].1, removed_tuple_cache[j].2)
                                        )
                                    );
                        }
                    }
                }

                removed_tuple_cache.clear();
                removed_tuple_cache.push(min);
            }
        }

        if min.2 >= N {
            counter += 1;
            println!("{:?}", counter);
            continue;
        } // Finish iteration if j >= N
        pq.insert((min.1.pow(3) + (min.2 + 1).pow(3), min.1, min.2 + 1)); // Insert next item with j + 1
    }

    println!("{:?}", distinct_integers);
    println!("Elapsed: {:.2?}", now.elapsed());
}