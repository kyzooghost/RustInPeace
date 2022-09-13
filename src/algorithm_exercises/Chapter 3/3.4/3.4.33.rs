#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p483 Exercises

// 3.4.33

// public int hashCode()
// {
//    int hash = 0;
//    int skip = Math.max(1, length()/8);
//    for (int i = 0; i < length(); i += skip)
//       hash = (hash * 37) + charAt(i);
//    return hash;
// }

// Bad hashcode?
// Skip many characters in a long string => Hash collision with many long strings with similar content

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug)]
pub struct STNode<T, U> {
    key: T,
    value: U,
}

pub struct CuckooHashingST<T, U> {
    total_num_of_keys: usize,

    table_one: Vec<Option<STNode<T, U>>>, // Table 1
    table_one_keys_count: usize,

    table_two: Vec<Option<STNode<T, U>>>, // Table 2
    table_two_keys_count: usize,
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug + Hash, 
    U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    CuckooHashingST<T, U> {
        pub fn num_of_keys(&self) -> usize {self.total_num_of_keys}
        pub fn isEmpty(&self) -> bool {self.total_num_of_keys == 0}

        pub fn new(_table_size: Option<usize>) -> Self {
            let mut table_one: Vec<Option<STNode<T, U>>> = Vec::new();
            let mut table_two: Vec<Option<STNode<T, U>>> = Vec::new();
            let table_size;

            // Default to 997
            match _table_size {
                None => {table_size = 997;},
                Some(i) => {table_size = i;}
            }

            for _ in 0..table_size { 
                table_one.push(None);
                table_two.push(None);
            }

            CuckooHashingST {
                total_num_of_keys: 0,
                table_one: table_one,
                table_one_keys_count: 0,
                table_two: table_two,
                table_two_keys_count: 0,
            }
        }

        fn _hash_one(&self, t: &T) -> usize {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            hasher.finish() as usize % self.table_one.len()
        }

        fn _hash_two(&self, t: &T) -> usize {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            hasher.finish() as usize % self.table_two.len()
        }

        fn _resize_table_one(&mut self, new_size: usize) {
            // Collect all entries in the table
            let mut entries: Vec<STNode<T, U>> = Vec::new();

            for entry in self.table_one.iter() {
                if let Some(node) = entry {
                    entries.push(STNode{key: node.key, value: node.value});
                }
            }

            // Create new table
            let mut table_one: Vec<Option<STNode<T, U>>> = Vec::new();
            for _ in 0..new_size {table_one.push(None)}
            self.table_one = table_one;

            // Rehash collected entries into new table
            for entry in entries {
                let hash = self._hash_one(&entry.key);
                match self.table_one[hash] {
                    None => {self.table_one[hash] = Some(entry);},
                    Some(_) => panic!("_resize_table_one - should not have preexisting entry")
                }
            }
        }

        fn _resize_table_two(&mut self, new_size: usize) {
            // Collect all entries in the table
            let mut entries: Vec<STNode<T, U>> = Vec::new();

            for entry in self.table_two.iter() {
                if let Some(node) = entry {
                    entries.push(STNode{key: node.key, value: node.value});
                }
            }

            // Create new table
            let mut table_two: Vec<Option<STNode<T, U>>> = Vec::new();
            for _ in 0..new_size {table_two.push(None)}
            self.table_two = table_two;


            // Rehash collected entries into new table
            for entry in entries {
                let hash = self._hash_two(&entry.key);
                match self.table_two[hash] {
                    None => {self.table_two[hash] = Some(entry);},
                    Some(_) => {panic!("_resize_table_two - should not have preexisting entry")}
                }
            }
        }

        pub fn keys(&self) -> Vec<&T> {
            let mut keys: Vec<&T> = Vec::new();

            for entry in self.table_one.iter() {
                if let Some(node) = entry {
                    keys.push(&node.key);
                }
            }

            for entry in &self.table_two {
                if let Some(node) = entry {
                    keys.push(&node.key);
                }
            }
            
            keys
        }

        pub fn get(&self, key: &T) -> Option<U> {
            let hash_one = self._hash_one(key);
            let hash_two = self._hash_two(key);

            // Attempt search in table 1, early return if search hit
            if let Some(node) = self.table_one[hash_one] {
                if &node.key == key {return Some(node.value)}
            }

            // Attempt search in table 2, return if search hit
            if let Some(node) = self.table_two[hash_two] {
                if &node.key == key {return Some(node.value)}
            } 

            return None;
        }

        pub fn contains(&self, key: &T) -> bool {
            match self.get(key) {
                None => {return false},
                Some(_) => {return true}
            }
        }

        pub fn put(&mut self, key: T, value: U) {
            self._put(key, value, 0)
        }

        fn _put(&mut self, key: T, value: U, recursion_depth: usize) {
            // If cuckoo hashing cycle more than 5 times, just rehash
            if recursion_depth > 5 {
                self._resize_table_one(self.table_one.len() * 2);
                self._resize_table_two(self.table_two.len() * 2);
                self.put(key, value);
                return
            }

            let hash_one_new_key = self._hash_one(&key);
            let hash_two_new_key = self._hash_two(&key);
            let new_node = Some(STNode{key: key, value: value});

            // CHECK IF KEY PRE-EXISTING, IF TRUE REPLACE

            // Attempt search in table 1, replace key and return if search hit
            if let Some(node) = &self.table_one[hash_one_new_key] {
                if node.key == key {
                    self.table_one[hash_one_new_key] = new_node;
                    return;
                }
            }

            // Attempt search in table 2, replace key and return if search hit
            if let Some(node) = &self.table_two[hash_two_new_key] {
                if node.key == key {
                    self.table_two[hash_two_new_key] = new_node;
                    return;
                }
            }

            // KEY NOT PRE-EXISTING, INSERT
            self.total_num_of_keys += 1;

            // If table 1 empty, if insert and return
            if let None = self.table_one[hash_one_new_key] {
                self.table_one[hash_one_new_key] = new_node;
                self.table_one_keys_count += 1;
                if self.table_one.len() < 2 & self.table_one_keys_count {self._resize_table_one(2 * self.table_one.len())}
                return;
            // Else table 1 has pre-existing element, 'replace' table 1 element and insert old table 1 element into table 2
            } else {
                let old_key = self.table_one[hash_one_new_key].as_mut().unwrap().key;
                let old_value = self.table_one[hash_one_new_key].as_mut().unwrap().value;
                let old_node = Some(STNode{key: old_key, value: old_value});
                self.table_one[hash_one_new_key] = new_node; // Replace table 1 element
                
                let hash_two_old_key = self._hash_two(&old_key);
                // If table 2 empty, insert and return
                if let None = self.table_two[hash_two_old_key] {
                    self.table_two[hash_two_old_key] = old_node;
                    self.table_two_keys_count += 1;
                    if self.table_two.len() < 2 & self.table_two_keys_count {self._resize_table_two(2 * self.table_two.len())}
                    return;
                // Else table 2 has pre-existing element, replace and make recursive call 
                } else {
                    let old_key_2 = self.table_two[hash_two_old_key].as_mut().unwrap().key;
                    let old_value_2 = self.table_two[hash_two_old_key].as_mut().unwrap().value;
                    self.table_two[hash_two_old_key] = old_node; // Replace table 2 element
                    self._put(old_key_2, old_value_2, recursion_depth + 1);
                }
            }
        }

        pub fn delete(&mut self, key: T) {
            let hash_one = self._hash_one(&key);
            let hash_two = self._hash_two(&key);

            // Attempt search in table 1, early return if search hit
            if let Some(node) = self.table_one[hash_one] {
                if node.key == key {
                    self.table_one[hash_one] = None;
                    self.total_num_of_keys -= 1;
                    self.table_one_keys_count -= 1;
                    return
                }
            }

            // Attempt search in table 2, return if search hit
            if let Some(node) = self.table_two[hash_two] {
                if node.key == key {
                    self.table_two[hash_two] = None;
                    self.total_num_of_keys -= 1;
                    self.table_two_keys_count -= 1;
                    return
                }
            }
        }
}

fn main() {
    let mut st = CuckooHashingST::new(Some(5));

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

    // for (i, letter) in vec.iter().enumerate() {
    //     st.put(letter.clone(), i.clone());
    // }

    for i in 0..1000 {
        st.put(i, i);
    }

    println!("{:?}", st.keys().len());
    println!("{:?}", st.keys());
    println!("{:?}", st.table_one.len());
    println!("{:?}", st.table_two.len());
}