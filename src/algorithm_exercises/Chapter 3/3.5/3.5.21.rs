#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p522

// 3.5.21 - Inverted Concordance

// Inverted concordance. Write a program InvertedConcordance that takes a concordance on standard input and puts the original string on standard output stream. Note : This computation is associated with a famous story having to do with the Dead Sea Scrolls. The team that discovered the original tablets enforced a secrecy rule that essentially resulted in their making public only a concordance. After a while, other researchers figured out how to invert the concordance, and the full text was eventually made public.

// InvertedST key = index, value = word, iterate by key - put words in same sequence

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