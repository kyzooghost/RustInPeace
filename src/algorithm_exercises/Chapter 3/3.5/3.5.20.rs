#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p522

// 3.5.20 - Concordance

// Write an ST client Concordance that puts on standard output a concordance of the strings in the standard input stream (see page 498).
// ST client, key => word, value => index, multiple values allowed per key

use std::hash::{Hash};
mod utils {pub mod LinearProbingHashTable;}
use utils::LinearProbingHashTable::LinearProbingHashTable as HashST;

pub struct MathSet<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug + Hash> {
    st: HashST<T, T>
}

impl<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug + Hash> MathSet<T> {
    pub fn add(&mut self, key: T) {
        self.st.put(key, key);
    }

    pub fn delete(&mut self, key: T) {
        self.st.delete(key);
    }

    pub fn contains(&self, key: &T) -> bool {
        self.st.contains(&key)
    }

    pub fn size(&self) -> usize {
        self.st.size()
    }

    pub fn isEmpty(&self) -> bool {
        self.st.size() == 0
    }

    // put any keys from a into the set that are not already there
    pub fn union(&mut self, set: MathSet<T>) {
        for key in set.st.keys() {
            if !self.contains(key) {self.add(key.clone())}
        }
    }

    // remove any keys from this set that are not in a
    pub fn intersection(&mut self, set: MathSet<T>) {
        // Not in set => delete key
        for key in set.st.keys() {
            if !self.contains(key) {self.delete(key.clone())}
        }
    }
}

fn main() {
}