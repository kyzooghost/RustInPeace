#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p389 Exercises

// 3.1.6 

// Give the number of calls to put() and get() issued 
// by FrequencyCounter, as a function of the number W of words 
// and the number D of distinct words in the input.

// In first `while (!StdIn.isEmpty()` loop - will call put() in each iteration. So call put() W times here
// Will call get() in each iteration involving a previous word, so W - D times

// 2nd outer loop - Iterate through keys, 2 get() calls each iteration. So 2D get() calls.

// put() - W + 1
// get() - W - D + 2D + 1 = W + D + !

// p363 ADT for ST
// p366 ST API
// p379 + 381 Ordered array ST implementation

mod utils {pub mod LinkedList;}
use utils::LinkedList::List as LinkedList;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Grade {
    F,
    D,
    C_Minus,
    C,
    C_Plus,
    B_Minus,
    B,
    B_Plus,
    A_Minus,
    A,
    A_Plus,
}

pub struct STNode<T, U> {
    key: T,
    value: U
}

pub struct OrderedSequentialSearchST<T, U> {
    size: usize,
    list: LinkedList<STNode<T, U>>,
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug, 
    U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    OrderedSequentialSearchST<T, U> {

    pub fn size(&self) -> usize {self.size}

    pub fn isEmpty(&self) -> bool {self.size == 0}

    pub fn new() -> Self {
        OrderedSequentialSearchST {
            size: 0,
            list: LinkedList::new()
        }
    }

    pub fn get(&self, key: &T) -> Option<U> {
        if self.isEmpty() {return None}

        // Linear search for key
        let iter = self.list.iter();

        for node in iter {
            // Search hit
            if &node.key == key {return Some(node.value)}
        }
        
        None
    }

    pub fn put(&mut self, key: T, value: U) {
        let rank = self.rank(key);

        // If match current key, change the value
        if rank < self.size && self.list.peek_at_index(rank).unwrap().key == key {
            let node = self.list.peek_mut_at_index(rank).unwrap();
            node.value = value;
            return
        }

        // Else insert (key, value) at index `rank`

        let new_node = STNode{
            key: key,
            value: value
        };

        self.list.insert_at_index(rank, new_node);

        self.size += 1;
    }

    // How many keys are below (if key not found) or equal (if key found)
    pub fn rank(&self, key: T) -> usize {
        if self.isEmpty() {return 0}

        let mut low = 0;
        let mut high = self.size - 1;
        let keys = self.keys();

        while low <= high {
            let mid = (low + high) / 2;
            if key < keys[mid] {
                if mid == 0 {break} // usize can't be negative, so break out of edge-case
                high = mid - 1
            }
            else if key > keys[mid] {low = mid + 1}
            else {return mid}
        }

        return low;
    }

    pub fn contains(&self, key: &T) -> bool {
        match self.get(key) {
            Some(_) => true,
            None => false
        }
    }

    pub fn keys(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let iter = self.list.iter();
        for node in iter {vec.push(node.key);}
        vec
    }

    pub fn min(&self) -> Option<T> {
        if self.size == 0 {return None}
        Some(self.list.peek_at_index(0).unwrap().key)   
    }

    pub fn max(&self) -> Option<T> {
        if self.size == 0 {return None}
        Some(self.list.peek_at_index(self.size - 1).unwrap().key)   
    }

    pub fn select(&self, index: usize) -> Option<T> {
        if self.size == 0 || index >= self.size {return None}
        Some(self.list.peek_at_index(index).unwrap().key)   
    }

    pub fn ceiling(&self, key: T) -> Option<T> {
        if self.size == 0 {return None}
        let rank = self.rank(key);
        Some(self.list.peek_at_index(rank).unwrap().key)
    }

    pub fn delete(&mut self, key: T) -> Option<T> {
        let rank = self.rank(key);

        // If match current key, change the value
        if rank < self.size && self.list.peek_at_index(rank).unwrap().key == key {
            let node = self.list.remove_at_index(rank);
            self.size -= 1;
            return Some(node.unwrap().key)
        }

        None
    }
}

fn main() {
}