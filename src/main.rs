#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p480 Exercises

// 3.4.11

// Give contents of linear-probing hash table when insert E A S Y Q U E S T I O N
// into an initially empty table of initial size M = 4, expanded by doubling whenever half full
// Use hash function 11 k % m

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct LinearProbingHashST<U> {
    num_of_keys: usize,
    size: usize,
    keys: Vec<Option<&'static str>>,
    values: Vec<Option<U>>
}

impl<U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    LinearProbingHashST<U> {

    pub fn size(&self) -> usize {self.size}

    pub fn num_of_keys(&self) -> usize {self.num_of_keys}

    pub fn isEmpty(&self) -> bool {self.num_of_keys == 0}

    pub fn new(size: usize) -> Self {
        let mut keys: Vec<Option<&str>> = Vec::new();
        let mut values: Vec<Option<U>> = Vec::new();

        for _ in 0..size {
            keys.push(None);
            values.push(None);
        }

        LinearProbingHashST {
            num_of_keys: 0,
            size: size,
            keys: keys,
            values: values
        }
    }

    pub fn get(&self, key: &str) -> Option<U> {
        // Start searching from index retrieved by hash
        let mut i = self._hash_letter(key);

        loop {
            match self.keys[i] {
                // Break the loop if nothing found 
                None => {break;},
                Some(stored_key_value) => {
                    // If search hit, return corresponding values[] entry
                    if stored_key_value == key {return self.values[i]}
                    // Increment to next slot (with modular maths) for next loop
                    i = (i + 1) % self.size();
                }
            }
        }

        // Search miss, return None
        None
    } 

    pub fn put(&mut self, key: &'static str, val: U) {
        if self.num_of_keys >= self.size / 2 {self._resize(2 * self.size)}

        let mut i = self._hash_letter(key);

        loop {
            match self.keys[i] {
                // Break the loop if nothing found 
                None => {break;},
                Some(stored_key_value) => {
                    // If search hit, return corresponding values[] entry
                    if stored_key_value == key {
                        self.values[i] = Some(val);
                        return
                    }
                    // Increment to next slot (with modular maths) for next loop
                    i = (i + 1) % self.size();
                }
            }
        }

        self.keys[i] = Some(key);
        self.values[i] = Some(val);
        self.num_of_keys += 1;
    }

    pub fn contains(&self, key: &str) -> bool {
        match self.get(key) {
            None => {return false},
            Some(_) => {return true}
        }
    }

    pub fn delete(&mut self, key: &str) {
        if !self.contains(key) {return}

        // Linear search for given key starting from hash index, then set to null
        let mut i = self._hash_letter(key);

        while key != self.keys[i].unwrap() {
            i = (i + 1) % self.size;
        }

        self.keys[i] = None;
        self.values[i] = None;

        // Iterate to next index from deleted
        // Reinsert all consecutive keys to the right of the deleted key/value pair
        // Otherwise may lose access to the key/value pair

        i = (i + 1) % self.size;

        loop {
            match self.keys[i] {
                None => {break;},
                Some(_) => {
                    let keyToRedo = self.keys[i].unwrap();
                    let valueToRedo = self.values[i].unwrap();
                    self.keys[i] = None;
                    self.values[i] = None;
                    self.num_of_keys -= 1;
                    self.put(keyToRedo, valueToRedo);
                    i = (i + 1) % self.size;
                }
            }
        }

        self.num_of_keys -= 1;
        if self.num_of_keys > 0 && self.num_of_keys < self.size / 8 {
            println!("resized in delete");
            self._resize(self.size / 2)
        }
    }

    // Circular dependency between _resize and put?
    fn _resize(&mut self, new_size: usize) {
        let mut st = LinearProbingHashST::new(new_size);

        for i in 0..self.size {
            match self.keys[i] {
                None => {},
                Some(stored_key) => {
                    st.put(stored_key, self.values[i].unwrap())
                }
            }
        }

        self.keys = st.keys;
        self.values = st.values;
        self.size = new_size;
    }

    // There is a type-mismatch issue in that if these associated functions take
    // &str type, and we have a broader generic definition of T, then these functions can't
    // be used for any other type for T
    fn _letter_to_number(&self, letter: &str) -> usize {
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

    fn _hash_letter(&self, letter: &str) -> usize {
        ( 11 * self._letter_to_number(&letter) ) % self.size
    }
}


fn main() {
    let mut st = LinearProbingHashST::new(4);

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

    println!("{:?}", st.keys);
    println!("{:?}", st.values);
}