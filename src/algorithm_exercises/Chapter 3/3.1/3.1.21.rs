#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p391 Exercises

// 3.1.22 Self-organizing search. 

// A self-organizing search algorithm is one that rearranges items 
// to make those that are accessed frequently likely to be found early in the search. 

// Modify your search implementation for Exercise 3.1.2 to perform the following action 
// on every search hit: move the key-value pair found to the beginning of the list, 
// moving all pairs between the beginning of the list and the vacated position 
// to the right one position. This procedure is called the move-to-front heuristic.

// Search hit -> Move key-value pair to beginning of list

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