#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p480 Exercises

// 3.4.2

// Develop an alternate implementation of SeparateChainingHashST
// that directly uses the linked-list code from SequentialSearchST.

mod utils {pub mod LinkedList;}
use utils::LinkedList::List as LinkedList;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct STNode<T, U> {
    key: T,
    value: U
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

    pub fn put(&mut self, key: T, value: U) {
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
            value: value
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
            let hash_of_key = self._hash(&key);
            self.chain_vec[hash_of_key].put(key, value);
        }

        pub fn get(&self, key: T) -> Option<U> {
            let hash_of_key = self._hash(&key);
            self.chain_vec[hash_of_key].get(&key)
        }
}

fn main() {
    let mut st = SeparateChainingHashST::new(None);

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

    println!("{:?}", st.get("Y"));
}

fn letter_to_number(letter: &str) -> usize {
    if letter == "A" {return 1}
    else if letter == "B" {return 2}
    else if letter == "C" {return 3}
    else if letter == "D" {return 4}
    else if letter == "E" {return 5}
    else if letter == "F" {return 6}
    else if letter == "G" {return 7}
    else if letter == "H" {return 8}
    else if letter == "I" {return 9}
    else if letter == "J" {return 10}
    else if letter == "K" {return 11}
    else if letter == "L" {return 12}
    else if letter == "M" {return 13}
    else if letter == "N" {return 14}
    else if letter == "O" {return 15}
    else if letter == "P" {return 16}
    else if letter == "Q" {return 17}
    else if letter == "R" {return 18}
    else if letter == "S" {return 19}
    else if letter == "T" {return 20}
    else if letter == "U" {return 21}
    else if letter == "V" {return 22}
    else if letter == "X" {return 23}
    else if letter == "W" {return 24}
    else if letter == "Y" {return 25}
    else if letter == "Z" {return 26}
    0
}

fn hash_letter(letter: &str) -> usize {
    ( 11 * letter_to_number(letter) ) % 5
}