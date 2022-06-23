#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p391 Exercises

// 3.1.23 Analysis of binary search. 

// Prove that the maximum number of compares used for a binary search 
// in a table of size N is precisely the number of bits in the binary representation of N, 
//because the operation of shifting 1 bit to the right converts the 
// binary representation of N into the binary representation of ⎣N/2⎦.

pub struct ArrayST<T, U> {
    size: usize,
    keys: Vec<T>,
    values: Vec<U>
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug, 
    U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    ArrayST<T, U> {

    pub fn size(&self) -> usize {self.size}

    pub fn isEmpty(&self) -> bool {self.size == 0}

    pub fn new() -> Self {
        ArrayST {
            size: 0,
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn get(&mut self, key: &T) -> Option<U> {
        if self.isEmpty() {return None}

        // Attempt linear search for key
        for (i, current_key) in self.keys.clone().iter().enumerate() {
            // If search hit, return value
            if key == current_key {
                let return_value = self.values[i].clone();

                // Re-insert element at beginning of list
                self.keys.remove(i);
                self.values.remove(i);
                self.keys.insert(0, *key);
                self.values.insert(0, return_value);

                return Some(return_value)
            }
        }

        None
    }

    pub fn put(&mut self, key: T, value: U) {
        // Attempt linear search for key
        for (i, current_key) in self.keys.iter().enumerate() {
            // If search hit, change value and return
            if &key == current_key {
                self.values[i] = value;

                // Re-insert element at beginning of list
                self.keys.remove(i);
                self.values.remove(i);
                self.keys.insert(0, key);
                self.values.insert(0, value);

                return
            }
        }

        // If linear search miss, push to arrays
        self.keys.push(key);
        self.values.push(value);
        self.size += 1;
    }

    pub fn contains(&self, key: &T) -> bool {
        match self.get(key) {
            Some(_) => true,
            None => false
        }
    }
}

fn main() {
}