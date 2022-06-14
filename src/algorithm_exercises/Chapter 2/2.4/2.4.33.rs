#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p332 Exercises

// 2.4.33 + 2.4.34

// Index priority-queue implementation. 
// Implement the basic operations in the index priority-queue API on page 320 by modifying Algorithm 2.6 as follows: 
// Change pq[] to hold indices, 
// add an array keys[] to hold the key values, 
// and add an array qp[] that is the inverse of pq[] — qp[i] gives the position of i in pq[] (the index j such that pq[j] is i). 
// Two implications here, i.) every index needs to be unique, this is not currently enforced by the API
// ii.) The indexes need to start from 0, and increment by 1 every time, this is also not currently enforced by the API
// i.) is implied by contains() method
// But ii.) ? 'with possible indices between 0 and maxN-1' A Vector can't handle 
// But the indexes won't be removed in order, 2 can be removed before 11, leaving a holey array
// So need to use a hashmap

// Then modify the code in Algorithm 2.6 to maintain these data structures. 
// Use the convention that qp[i] = -1 if i is not on the queue, 
// and include a method contains() that tests this condition. 
// You need to modify the helper methods exch() and less() but not sink() or swim().

// Gg, the edge cases with change() and delete() got me, and dealing with the holey array issue

use std::collections::HashMap;

pub struct MinPQ<T> {
    heap: Vec<Option<(usize, T)>>,
    index_to_heap_position: HashMap<usize, usize>,
    size: usize,
    max_size: usize
}

impl<T: Copy + Ord + std::fmt::Debug> MinPQ<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new(maxN: usize) -> Self {
        MinPQ { 
            size: 0, 
            max_size: maxN,
            heap: vec![None], 
            index_to_heap_position: HashMap::new() }
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn contains(&self, key: usize) -> bool {
        return self.index_to_heap_position.contains_key(&key)
    }

    pub fn min(&self) -> Option<(usize, T)> {
        if self.size == 0 {return None}
        let min = self.heap[1].unwrap();
        Some((min.0, min.1))
    }

    pub fn insert(&mut self, key: usize, element: T) {
        // Probably shouldn't panic here, and return a catchable error instead
        if self.size >= self.max_size {panic!("max_size reached")}
        if key >= self.max_size {panic!("key >= max_size")}
        if self.contains(key) {panic!("Previously inserted key")}
        self.size += 1;
        self.heap.push(Some((key, element)));
        self.index_to_heap_position.insert(key, self.size);
        self.swim(self.size);
    }

    pub fn change(&mut self, key: usize, element: T) {
        if !self.contains(key) {panic!("Key not present")}
        let heap_position = self.index_to_heap_position.get(&key).unwrap().clone();
        self.heap[heap_position] = Some((key, element));

        // Swim up if parent is smaller
        if heap_position > 1 && element < self.heap[heap_position / 2].unwrap().1 {self.swim(heap_position)}
        else {self.sink(heap_position)}

        // self.sink(heap_position);
    }

    pub fn delete(&mut self, key: usize) -> Option<(usize, T)> {
        if !self.contains(key) {return None}

        let heap_position = self.index_to_heap_position.get(&key).unwrap().clone();

        // Bug here, also need to change index_to_heap_position mapping for element previously at self.size
        self.index_to_heap_position.insert(self.heap[self.size].unwrap().0, heap_position);
        self.heap.swap(heap_position, self.size);


        let element_to_remove = self.heap.pop().unwrap().unwrap();
        self.size -= 1;
        self.sink(heap_position);

        // Remove index from index_to_heap_position HashMap
        let key_to_remove = element_to_remove.0;
        self.index_to_heap_position.remove_entry(&key_to_remove);

        Some((element_to_remove.0, element_to_remove.1))
    }

    pub fn delMin(&mut self) -> Option<(usize, T)> {
        if self.size == 0 {return None}

        self.heap.swap(1, self.size);
        let max = self.heap.pop().unwrap().unwrap();
        self.size -= 1;
        self.sink(1);

        // Remove index from index_to_heap_position HashMap
        let key_to_remove = max.0;
        self.index_to_heap_position.remove_entry(&key_to_remove);

        Some((max.0, max.1))
    }

    fn swim(&mut self, mut index: usize) {
        while index > 1 && self.heap[index / 2].unwrap().1 > self.heap[index].unwrap().1 {
            
            // Swap heap position on index_to_heap_position
            self.index_to_heap_position.insert(self.heap[index].unwrap().0, index / 2);
            self.index_to_heap_position.insert(self.heap[index / 2].unwrap().0, index);

            // Classic swim code
            self.heap.swap(index, index / 2);
            index = index / 2;
        }
    }

    fn sink(&mut self, mut index: usize) {
        while 2*index <= self.size {
            let mut j = 2 * index;
            if j < self.size && self.heap[j].unwrap().1 > self.heap[j + 1].unwrap().1 {j += 1}
            if self.heap[index].unwrap().1 <= self.heap[j].unwrap().1 {break}

            // Swap heap position on index_to_heap_position
            self.index_to_heap_position.insert(self.heap[index].unwrap().0, j);
            self.index_to_heap_position.insert(self.heap[j].unwrap().0, index);

            // Classic sink code
            self.heap.swap(index, j);
            index = j;
        }   
    }
}

// Q

fn main() {
    let vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    let mut pq = MinPQ::new(12);
    let mut i = 0;
    for element in vec {
        pq.insert(i, element);
        i += 1;
    }  

    pq.change(0, "R");
    pq.change(1, "Y");
    pq.change(2, "F");
    pq.delete(2);
    pq.change(3, "Z");

    pq.delete(1);
    pq.delete(3);
    pq.delete(5);
    pq.delete(9);
    pq.delete(4);
    pq.delete(5);
    pq.delete(6);

    while pq.size() > 0 {
        println!("{:?}", pq.delMin());
    }
}