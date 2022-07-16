#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p482 Exercises

// 3.4.19
// Implement keys() for SeparateChainingHashST and LinearProbingHashST.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct LinearProbingHashST<T, U> {
    num_of_keys: usize,
    size: usize,
    keys: Vec<Option<T>>,
    values: Vec<Option<U>>,
    average_probe_threshold: usize
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug + Hash
, U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    LinearProbingHashST<T, U> {

    pub fn size(&self) -> usize {self.size}

    pub fn num_of_keys(&self) -> usize {self.num_of_keys}

    pub fn isEmpty(&self) -> bool {self.num_of_keys == 0}

    pub fn new(_size: usize, _average_probe_threshold: usize) -> Self {
        let mut keys: Vec<Option<T>> = Vec::new();
        let mut values: Vec<Option<U>> = Vec::new();

        for _ in 0.._size {
            keys.push(None);
            values.push(None);
        }

        LinearProbingHashST {
            num_of_keys: 0,
            size: _size,
            keys: keys,
            values: values,
            average_probe_threshold: _average_probe_threshold
        }
    }

    pub fn get(&self, key: &T) -> Option<U> {
        // Start searching from index retrieved by hash
        let mut i = self._hash(key);

        loop {
            match self.keys[i] {
                // Break the loop if nothing found 
                None => {break;},
                Some(stored_key_value) => {
                    // If search hit, return corresponding values[] entry
                    if &stored_key_value == key {return self.values[i]}
                    // Increment to next slot (with modular maths) for next loop
                    i = (i + 1) % self.size();
                }
            }
        }

        // Search miss, return None
        None
    } 

    pub fn put(&mut self, key: T, val: U) {
        if self.num_of_keys >= self.size / 2 || self.num_of_keys / self.size > self.average_probe_threshold {
            self._resize(2 * self.size)
        }

        let mut i = self._hash(&key);

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

    pub fn contains(&self, key: &T) -> bool {
        match self.get(key) {
            None => {return false},
            Some(_) => {return true}
        }
    }

    pub fn delete(&mut self, key: T) {
        if !self.contains(&key) {return}

        // Linear search for given key starting from hash index, then set to null
        let mut i = self._hash(&key);

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

    pub fn keys(&self) -> Vec<&T> {
        let mut keys: Vec<&T> = Vec::new();

        for key in self.keys.iter() {
            match key {
                None => {},
                Some(stored_key) => {
                    keys.push(stored_key)
                }
            }
        }

        keys
    }

    // Circular dependency between _resize and put?
    fn _resize(&mut self, new_size: usize) {
        let mut st = LinearProbingHashST::new(new_size, self.average_probe_threshold);

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
        ( 16 * self._letter_to_number(&letter) ) % self.size
    }

    // Expect to disperse key uniformly among all possible 64-bit result values
    fn _hash(&self, t: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        let mut hash = hasher.finish() as usize;
        let lgM = (self.size as f64).ln() as usize;

        // Distribute key value using prime larger than M
        // Distribute values equally among values less than each prime
        if lgM < 26 {hash = hash % self._prime(lgM + 5);}
        // Map five of those values to value less than M
        hash % self.size
    }

    // Return smallest prime greater than each power of 2
    fn _prime(&self, m: usize) -> usize {
        match m {
            5 => {return 31},
            6 => {return 61},
            7 => {return 127},
            8 => {return 251},
            9 => {return 509},
            10 => {return 1021},
            11 => {return 2039},
            12 => {return 4093},
            13 => {return 8191},
            14 => {return 16381},
            15 => {return 32749},
            16 => {return 65521},
            17 => {return 131071},
            18 => {return 262139},
            19 => {return 524287},
            20 => {return 1048573},
            21 => {return 2097143},
            22 => {return 4194301},
            23 => {return 8388593},
            24 => {return 16777213},
            25 => {return 33554393},
            26 => {return 67108859},
            27 => {return 134217689},
            28 => {return 268435399},
            29 => {return 536870909},
            30 => {return 1073741789},
            31 => {return 2147483647},
            _ => {panic!("did not provide value between 5 and 31 (inclusive)")}
        }
    }
}

fn main() {
    let mut st = LinearProbingHashST::new(16, 5);

    let vec = vec![
        "S",
        "E",
        "A",
        "R",
        "C",
        "H",
        "E",
        "X",
        "A",
        "M",
        "P",
        "L",
        "E",
    ];

    for (i, letter) in vec.iter().enumerate() {
        st.put(letter.clone(), i.clone());
    }

    println!("{:?}", st.keys());
    st.delete("C");
    println!("{:?}", st.keys());

}