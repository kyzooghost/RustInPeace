#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p483 Exercises

// 3.4.28

// Double hashing. Modify LinearProbingHashST to use a second hash function 
// to define the probe sequence. Specifically,replace (i + 1) % M (both occurrences) 
// by (i + k) % M
// where k is a non zero key-dependent integer that is relatively prime to M. 
// Note : You may meet the last condition by assuming that M is prime. 
// Give a trace of the process of inserting the keys E A S Y Q U E S T I O N
// in that order into an initially empty table of size M = 11, 
// using the hash functions described in the previous exercise. 
// Give the average number of probes for random search hit and search miss in this table.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct LinearProbingHashST<T, U> {
    num_of_keys: usize,
    size: usize,
    keys: Vec<Option<T>>,
    values: Vec<Option<U>>,
    tombstone_count: usize
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug + Hash
, U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    LinearProbingHashST<T, U> {

    pub fn size(&self) -> usize {self.size}

    pub fn num_of_keys(&self) -> usize {self.num_of_keys}

    pub fn isEmpty(&self) -> bool {self.num_of_keys == 0}

    pub fn new(_size: usize) -> Self {
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
            tombstone_count: 0
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
                    i = (i + self._hash_two(key)) % self.size;
                }
            }
        }

        // Search miss, return None
        None
    } 

    pub fn put(&mut self, key: T, val: U) {
        if self.num_of_keys + self.tombstone_count >= self.size / 2 && self.size != self._prime(31) {
            self._resize(self._prime(3 + self._lgSize()));
            // self._resize(2 * self.size);
        }

        let mut i = self._hash(&key);

        loop {
            match self.keys[i] {
                // Break the loop if nothing found 
                None => {break;},
                Some(stored_key_value) => {
                    // If search hit, return corresponding values[] entry
                    if stored_key_value == key {
                        match self.values[i] {
                            // Search hit with tombstone, so we are removing a tombstone
                            None => {self.tombstone_count -= 1;}
                            _ => {}
                        }

                        self.values[i] = Some(val); // Can overwrite null value here
                        return
                    }
                    // Increment to next slot (with modular maths) for next loop
                    i = (i + self._hash_two(&key)) % self.size;
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
            i = (i + self._hash_two(&key)) % self.size;
        }

        // self.keys[i] = None;
        self.tombstone_count += 1;
        self.values[i] = None;

        // Lazy deletion - necessary for double hashing (otherwise you can lose keys)
        // E.g. "S" and "E" have a hash collision, "S" inserted then "E", delete "S"
        // Can no longer access "E" in eager deletion (need tombstone to get to E)

        /* EAGER DELETION */

        // Iterate to next index from deleted
        // Reinsert all consecutive keys to the right of the deleted key/value pair
        // Otherwise may lose access to the key/value pair

        // i = (i + self._hash_two(&key)) % self.size;

        // loop {
        //     match self.keys[i] {
        //         None => {break;},
        //         Some(_) => {
        //             let keyToRedo = self.keys[i].unwrap();
        //             let valueToRedo = self.values[i].unwrap();
        //             self.keys[i] = None;
        //             self.values[i] = None;
        //             self.num_of_keys -= 1;
        //             self.put(keyToRedo, valueToRedo);
        //             i = (i + self._hash_two(&key)) % self.size;
        //         }
        //     }
        // }

        self.num_of_keys -= 1;

        if self.num_of_keys > 0 && self.num_of_keys < self.size / 8 {
            self._resize(self._prime(self._lgSize()));
            // self._resize(self.size / 2)
        }
    }

    pub fn keys(&self) -> Vec<&T> {
        let mut keys: Vec<&T> = Vec::new();

        for key in self.keys.iter() {
            match key {
                None => {},
                Some(stored_key) => {
                    match self.get(stored_key) {
                        None => {}, // Tombstone
                        Some(_) => {keys.push(stored_key)
                        }
                    }
                }
            }
        }

        keys
    }

    // Circular dependency between _resize and put?
    fn _resize(&mut self, new_size: usize) {

        println!("RESIZE to {:?} at {:?} keys", new_size, self.num_of_keys);
        let mut st = LinearProbingHashST::new(new_size);

        for i in 0..self.size {
            match self.keys[i] {
                None => {},
                Some(stored_key) => {
                    match self.values[i] {
                        // Tombstone (value is null) => don't re-insert => removed a tombstone
                        None => {self.tombstone_count -= 1;},
                        Some(stored_value) => {st.put(stored_key, stored_value)}
                    }
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
        let lgM = self._lgSize();

        // Distribute key value using prime larger than M
        // Distribute values equally among values less than each prime
        if lgM < 26 {hash = hash % self._prime(lgM + 5);}
        // Map five of those values to value less than M
        hash % self.size
    }

    fn _hash_two(&self, t: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        t.hash(&mut hasher);
        let hash = hasher.finish() as usize % self.size;

        if hash == 0 {
            return self.size + 1
        } else {
            return hash
        }
    }

    pub fn hash_two(&self, t: &T) -> usize {
        self._hash_two(t)
    }

    pub fn hash(&self, t: &T) -> usize {
        self._hash(t)
    }

    // Return smallest prime greater than each power of 2
    // Use to uniformly distribute keys after resize
    fn _prime(&self, m: usize) -> usize {
        match m {
            0 => {return 1},
            1 => {return 1},
            2 => {return 3},
            3 => {return 7},
            4 => {return 13},
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

    fn _lgSize(&self) -> usize {
        (self.size as f64).ln() as usize
    }
}

fn main() {
    let mut st = LinearProbingHashST::new(16);

    let vec = vec![
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        // "S",
        // "E",
        // "A",
        // "R",
        // "C",
        // "H",
        // "E",
        // "X",
        // "A",
        // "M",
        // "P",
        // "L",
        // "E",
        // "B",
        // "T",
        // "X",
        // "Z",
        // "I",
        // "J",
        // "M",
    ];

    for (i, letter) in vec.iter().enumerate() {
        st.put(letter.clone(), i.clone());
    }

    // println!("{:?}", st.size);

    st.delete(1);
    st.delete(2);
    st.delete(3);
    st.delete(4);
    st.delete(5);
    st.delete(6);
    st.delete(7);
    st.delete(8);
    st.delete(9);
    st.delete(10);
    st.delete(11);
    st.delete(12);
    st.delete(13);
    st.delete(14);
    st.delete(15);
    st.put(1, 1);
    st.put(2, 1);

    // st.delete("C");
    // st.delete("S");
    // st.delete("A");

    // // println!("{:?}", st.keys());
    // // println!("{:?}", st.num_of_keys);
    // println!("{:?}", st.size);

    // println!("------");
    // st.delete("H");
    // st.delete("M");
    // // println!("{:?}", st.keys());
    // // println!("{:?}", st.num_of_keys);
    // println!("{:?}", st.size);

    // println!("------");
    // st.delete("E");
    // st.delete("R");
    // println!("keys - {:?}, size - {:?}", st.num_of_keys ,st.size);

    // st.delete("X");
    // println!("keys - {:?}, size - {:?}", st.num_of_keys ,st.size);
    // st.delete("L");
    // // println!("{:?}", st.keys());
    // // println!("{:?}", st.num_of_keys);
    // println!("{:?}", st.size);


}