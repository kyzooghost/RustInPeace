#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 3.5.12
// Modify LookupCSV (p495) to associate with each key all values that appear in a key-value pair with that key in the input (not just the most recent, as in the associative-array abstraction).

// Parse amino.csv
// Obtain key query from standard input, print all values

// Dependencies
// csv = "1.1"

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::error::Error;
use std::fs::File;
use std::process;
use std::io;

pub struct LinearProbingHashTable<T, U> {
    num_of_keys: usize,
    size: usize,
    keys: Vec<Option<T>>,
    values: Vec<Option<U>>,
    tombstone_count: usize
}

impl<T: Clone + PartialEq + std::fmt::Debug + Hash
, U: Clone + PartialEq + std::fmt::Debug + Hash> 
    LinearProbingHashTable<T, U> {

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

        LinearProbingHashTable {
            num_of_keys: 0,
            size: _size,
            keys: keys,
            values: values,
            tombstone_count: 0
        }
    }

    pub fn get(&self, key: &T) -> Option<&U> {
        // Start searching from index retrieved by hash
        let mut i = self._hash(key);

        loop {
            match &self.keys[i] {
                // Break the loop if nothing found 
                None => {break;},
                Some(stored_key_value) => {
                    // If search hit, return corresponding values[] entry
                    if stored_key_value == key {return self.values[i].as_ref()}
                    // Increment to next slot (with modular maths) for next loop
                    i = (i + 1) % self.size();
                }
            }
        }

        // Search miss, return None
        None
    } 

    pub fn getAll(&self, key: &T) -> Vec<&U> {
        // Start searching from index retrieved by hash
        let mut i = self._hash(key);
        let mut vec = Vec::new();

        loop {
            match &self.keys[i] {
                // Break the loop if nothing found 
                None => {break;},
                Some(stored_key_value) => {
                    // If search hit, return corresponding values[] entry
                    if stored_key_value == key {vec.push(self.values[i].as_ref().unwrap())}
                    // Increment to next slot (with modular maths) for next loop
                    i = (i + 1) % self.size();
                }
            }
        }

        // Search miss, return None
        vec
    } 

    pub fn contains(&self, key: &T) -> bool {
        match self.get(key) {
            None => {return false},
            Some(_) => {return true}
        }
    }

    pub fn keys(&self) -> Vec<&T> {
        let mut keys: Vec<&T> = Vec::new();

        for i in 0..self.size {
            if let Some(_) = self.values[i] {
                keys.push(self.keys[i].as_ref().unwrap())
            }
        }

        keys
    }

    // Handle case where we found preexisting same key - only early return if we are replacing tombstone, otherwise keep incrementing.
    pub fn put(&mut self, key: T, val: U) {
        let mut i = self._hash(&key);

        // Find index to insert (key, val)
        loop {
            if let Some(stored_key_value) = &self.keys[i] {
                // If found preexisting same key
                if stored_key_value == &key {
                    // Check if hit tombstone => if true we are replacing tombstone with live entry
                    if let None = self.values[i] {
                        self.tombstone_count -= 1;
                        self.values[i] = Some(val);
                        return
                    }
                }
                // Else found different key, increment to next slot
                i = (i + 1) % self.size();
            // Found empty slot, insert
            } else {
                break;
            }
        }

        self.keys[i] = Some(key);
        self.values[i] = Some(val);
        self.num_of_keys += 1;

        // Resize if lambda >= 0.5, or too many tombstones
        if self.size < 2 * self.num_of_keys || self.tombstone_count * 4 > self.num_of_keys {
            self._resize(2 * self.size)
        }
    }

    // Modification - need to loop through entire set of keys and values
    // If same key, should be in the same chain in a linear probing structure
    pub fn delete(&mut self, key: T) {
        // Find index containing key
        let mut i = self._hash(&key);

        loop {
            match &self.keys[i] {
                // Found empty key slot and hence end of current chain (which will contain same keys)
                None => {break},
                Some(stored_key_value) => {
                    // Search hit => Soft delete, then increment to next element in the chain
                    if stored_key_value == &key {
                        // Soft delete, do not set `self.keys[i] = None`
                        self.tombstone_count += 1;
                        self.num_of_keys -= 1;
                        self.values[i] = None;
                    }
                    i = (i + 1) % self.size();
                }
            }
        }

        if self.tombstone_count * 4 > self.num_of_keys || ( self.num_of_keys > 0 && self.num_of_keys < self.size / 8 ){
            self._resize(self.size / 2)
        }
    }

    fn _resize(&mut self, new_size: usize) {
        let mut st = LinearProbingHashTable::new(new_size);
        self.tombstone_count = 0;

        for i in 0..self.size {
            let stored_key = self.keys[i].take();
            let stored_value = self.values[i].take();

            if let Some(_) = stored_value.clone() {
                st.put(stored_key.unwrap(), stored_value.unwrap());
            }

            // if let Some(stored_value) = self.values[i] {
            //     st.put(self.keys[i].unwrap(), stored_value);
            // }
        }

        self.size = st.size;
        self.keys = st.keys;
        self.values = st.values;
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

fn run() -> Result<(), Box<dyn Error>> {
    // Parse CSV file contents into Symbol Table
    let FILE_PATH: String = "src/algorithm_exercises/Chapter 3/3.5/amino.csv".to_string();
    let CSV_FILE = File::open(FILE_PATH)?;
    let mut reader = csv::Reader::from_reader(CSV_FILE);
    let mut st = LinearProbingHashTable::new(10);

    for result in reader.records().into_iter() {
        let record = result?;
        let key = record.get(1).unwrap().to_string().to_uppercase();
        let value = record.get(0).unwrap().to_string();
        st.put(key, value);
    }

    // stdin query for key
    println!("Enter amino acid (abbreviated to first three letters, e.g. 'ser' for Serine) to query for: ");

    loop {
        let mut query = String::new();
        io::stdin().read_line(&mut query)?;
        query = query.trim().to_string().to_uppercase();
        if st.contains(&query) {
            println!("{:?}", st.getAll(&query));
            break;
        } else {
            println!("Invalid query '{:?}'", query);
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

// ./target/debug/my-project < "src/algorithm_exercises/Chapter 3/3.5/amino.csv"
// fn main() {
//     let mut reader = csv::Reader::from_reader(io::stdin());
//     // Loop over each record.
//     for result in reader.records() {
//         let record = result.expect("a CSV record");
//         println!("{:?}", record);
//     }
// }