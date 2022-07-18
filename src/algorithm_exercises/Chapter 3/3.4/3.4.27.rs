#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p483 Exercises

// 3.4.27

// Double probing. 

// Modify SeparateChainingHashST to use a second hash function and pick the shorter of the two lists. 
// Give a trace of the process of inserting the keys E A S Y Q U E S T I O N into an initially empty 
// table of size M = 3. 
// Hash function 1 - 11 k % M
// Hash function 2 - 17 k % M

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
            (11 + hasher.finish() as usize) % self.num_of_chains
        }

        fn _hash_two(&self, t: &T) -> usize {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            (17 + hasher.finish() as usize) % self.num_of_chains
        }

        pub fn put(&mut self, key: T, value: U) {

            let hash_one = self._hash(&key);
            let hash_two = self._hash_two(&key);

            // If doesn't contain key, insert
            if !self.contains(&key) {
                self.num_of_keys += 1;

                if self.chain_vec[hash_one].size <= self.chain_vec[hash_two].size {
                    // List 1 <= List 2, put in List 1
                    self.chain_vec[hash_one].put(key, value, self.num_of_keys);
                } else {
                    // List 2 < List 1, put in List 2
                    self.chain_vec[hash_two].put(key, value, self.num_of_keys);
                }
            // Else contains key, find and replace key
            } else {
                if self.chain_vec[hash_one].contains(&key) {
                    self.chain_vec[hash_one].put(key, value, self.num_of_keys);
                } else {
                    self.chain_vec[hash_two].put(key, value, self.num_of_keys);
                }
            }
            
            // Probably inefficient to use self.get() method instead of using a return value from
            // SequentialSearchST.put(), but oh well
            // match self.get(&key) {
            //     None => {self.num_of_keys += 1;}
            //     Some(_) => {}
            // }

            // let num_of_keys = self.num_of_keys();
            // let hash_one = self._hash(&key);
            // let hash_two = self._hash_two(&key);

            // if hash_one < hash_two {
            //     self.chain_vec[hash_one].put(key, value, num_of_keys);
            // } else {
            //     self.chain_vec[hash_two].put(key, value, num_of_keys);
            // }

        }

        pub fn get(&self, key: &T) -> Option<U> {

            // let hash_of_key = self._hash(&key);
            // self.chain_vec[hash_of_key].get(&key)

            let hash_one = self._hash(key);
            let hash_two = self._hash_two(key);

            if self.chain_vec[hash_one].size <= self.chain_vec[hash_two].size {
                // List 1 <= List 2, search in List 1
                match self.chain_vec[hash_one].get(key) {
                    Some(value) => {return Some(value)},
                    None => {
                        // Search miss in list 1, search in list 2
                        return self.chain_vec[hash_two].get(key)
                    }
                }
            } else {
                // List 2 < List 1, search in List 2
                match self.chain_vec[hash_two].get(key) {
                    Some(value) => {return Some(value)},
                    None => {
                        // Search miss in list 2, search in list 1
                        return self.chain_vec[hash_one].get(key)
                    } 
                }
            }
        }

        pub fn delete(&mut self, key: T) {
            match self.get(&key) {
                None => {return}
                Some(_) => {
                    self.num_of_keys -= 1;
                    let hash_one = self._hash(&key);
                    let hash_two = self._hash_two(&key);

                    if self.chain_vec[hash_one].contains(&key) {
                        self.chain_vec[hash_one].delete(key)
                    } else {
                        self.chain_vec[hash_two].delete(key)
                    }
                }
            }

            // let hash_of_key = self._hash(&key);
            // self.chain_vec[hash_of_key].delete(key)
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

        pub fn keys(&self) -> Vec<T> {
            let mut keys: Vec<T> = Vec::new();

            for chain in &self.chain_vec {
                keys.extend_from_slice(&chain.keys())
            }
            
            keys
        }

        pub fn resize(&mut self, new_size: usize) {
            let mut st = SeparateChainingHashST::new(Some(new_size));

            for key in self.keys() {
                st.put(key, st.get(&key).unwrap());
            }

            self.chain_vec = st.chain_vec;
            self.num_of_keys = st.num_of_keys;
            self.num_of_chains = st.num_of_chains;
        }

        pub fn contains(&self, key: &T) -> bool {
            match self.get(key) {
                None => {return false},
                Some(_) => {return true}
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

    println!("{:?}", st.keys());
    st.deleteKeysAboveInsertionIndex(5);
    // st.delete("A");
    // st.delete("E");
    // st.delete("O");
    // st.delete("I");
    // st.delete("N");
    println!("{:?}", st.keys());

}