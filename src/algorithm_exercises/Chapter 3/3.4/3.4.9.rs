#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p480 Exercises

// 3.4.9
// Implement an eager delete() method for SeparateChainingHashST.

mod utils {pub mod LinkedList;}
use utils::LinkedList::List as LinkedList;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct STNode<T, U> {
    key: T,
    value: U,
    entries_at_insertion: usize
}

// SequentialSearchST is the linked list data structure representing each 'chain'
pub struct SequentialSearchST<T, U> {
    size: usize,
    list: LinkedList<STNode<T, U>>,
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug + Hash, 
    U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    SequentialSearchST<T, U> {

    pub fn size(&self) -> usize {self.size}

    pub fn isEmpty(&self) -> bool {self.size == 0}

    pub fn new() -> Self {
        SequentialSearchST {
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

    pub fn getEntriesAtInsertion(&self, key: &T) -> Option<usize> {
        if self.isEmpty() {return None}

        // Linear search for key
        let iter = self.list.iter();

        for node in iter {
            // Search hit
            if &node.key == key {return Some(node.entries_at_insertion)}
        }
        
        None
    }

    pub fn findIndex(&self, key: &T) -> Option<usize> {
        if self.isEmpty() {return None}

        // Linear search for key
        let iter = self.list.iter();

        for (i, node) in iter.enumerate() {
            // Search hit
            if &node.key == key {return Some(i)}
        }
        
        None
    }

    pub fn put(&mut self, key: T, value: U, _entries_at_insertion: usize) {
        for node in self.list.iter_mut() {
            // Search hit
            if node.key == key {
                node.value = value;
                return
            }
        }

        // Else search miss, insert at end of linked list
        let new_node = STNode{
            key: key,
            value: value,
            entries_at_insertion: _entries_at_insertion
        };

        self.list.insert_at_head(new_node);
        self.size += 1;
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

    pub fn delete(&mut self, key: T) {
        let index = self.findIndex(&key);

        match index {
            None => {return},
            Some(index) => {
                self.list.remove_at_index(index);
                self.size -= 1;
                return
            }
        }
    }
}

pub struct SeparateChainingHashST<T, U> {
    num_of_keys: usize,
    num_of_chains: usize,
    chain_vec: Vec<SequentialSearchST<T, U>>
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug + Hash, 
    U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    SeparateChainingHashST<T, U> {
        pub fn num_of_keys(&self) -> usize {self.num_of_keys}
        pub fn num_of_chains(&self) -> usize {self.num_of_chains}
        pub fn isEmpty(&self) -> bool {self.num_of_keys == 0}

        pub fn new(_num_of_chains: Option<usize>) -> Self {
            let mut chain_vec: Vec<SequentialSearchST<T, U>> = Vec::new();
            let unwrapped_num_of_chains;

            // Default to 997
            match _num_of_chains {
                None => {unwrapped_num_of_chains = 997;},
                Some(i) => {unwrapped_num_of_chains = i;}
            }

            for _ in 0..unwrapped_num_of_chains { chain_vec.push(SequentialSearchST::new()) }

            SeparateChainingHashST {
                num_of_keys: 0,
                num_of_chains: unwrapped_num_of_chains,
                chain_vec: chain_vec
            }
        }

        // Expect to disperse key uniformly among all possible 64-bit result values
        fn _hash(&self, t: &T) -> usize {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            hasher.finish() as usize % self.num_of_chains
        }

        pub fn put(&mut self, key: T, value: U) {
            // Probably inefficient to use self.get() method instead of using a return value from
            // SequentialSearchST.put(), but oh well
            match self.get(key) {
                None => {self.num_of_keys += 1;}
                Some(_) => {}
            }

            let num_of_keys = self.num_of_keys();
            let hash_of_key = self._hash(&key);
            self.chain_vec[hash_of_key].put(key, value, num_of_keys);
        }

        pub fn get(&self, key: T) -> Option<U> {
            let hash_of_key = self._hash(&key);
            self.chain_vec[hash_of_key].get(&key)
        }

        pub fn delete(&mut self, key: T) {
            match self.get(key) {
                None => {}
                Some(_) => {self.num_of_keys -= 1;}
            }

            let hash_of_key = self._hash(&key);
            self.chain_vec[hash_of_key].delete(key)
        }

        pub fn deleteKeysAboveInsertionIndex(&mut self, index_threshold: usize) {
            // O(N) algorithm to iterate through each entry

            // Iterate through each chain
            for i in 0..self.num_of_chains {
                let mut keys_to_be_deleted: Vec<T> = Vec::new();

                for node in self.chain_vec[i].list.iter() {
                    if node.entries_at_insertion > index_threshold {
                        keys_to_be_deleted.push(node.key);
                    }
                }

                // Delete in descending order (otherwise index unstable if deleted in ascending order)
                while !keys_to_be_deleted.is_empty() {
                    self.chain_vec[i].delete(keys_to_be_deleted.pop().unwrap());
                    self.num_of_keys -= 1;
                }
            }
        }
}

fn main() {
    let mut st = SeparateChainingHashST::new(Some(10));

    let vec = vec![
        "E",
        "A",
        "S",
        "Y",
        "Q",
        "U",
        "E",
        "S",
        "T",
        "I",
        "O",
        "N",
    ];

    for (i, letter) in vec.iter().enumerate() {
        st.put(letter.clone(), i.clone());
    }

    println!("{:?}", st.num_of_keys());
    // st.deleteKeysAboveInsertionIndex(5);
    st.delete("A");
    st.delete("E");
    println!("{:?}", st.num_of_keys());

    for i in 0..10 {
        println!("{:?}", st.chain_vec[i].keys());
    }
}