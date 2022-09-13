#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::hash::{Hash};
mod utils {pub mod LinearProbingHashTable; pub mod RedBlackTree;}
use utils::LinearProbingHashTable::LinearProbingHashTable as HashST;
use utils::RedBlackTree::RedBlackBST as ST;

// 3.5.1 Implement SET and HashSET as “wrapperclass” clients of ST and HashST, respectively (provide dummy values and ignore them).

pub struct Set<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> {
    st: ST<T,U>
}

pub struct HashSet<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug + Hash, U: Clone + PartialOrd + PartialEq + Copy + Hash> {
    st: HashST<T,U>
}

impl<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> Set<T, U> {
    pub fn add(&mut self, key: T, val: U) {
        self.st.put(key, val);
    }

    pub fn delete(&mut self, key: T) {
        self.st.delete(key);
    }

    pub fn contains(&self, key: T) -> bool {
        if let None = self.st.get(key) {return false}
        else {return true}
    }

    pub fn size(&self) -> usize {
        self.st.size()
    }

    pub fn isEmpty(&self) -> bool {
        self.st.is_empty()
    }
}

impl<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug + Hash, U: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug + Hash> HashSet<T, U> {
    pub fn add(&mut self, key: T, val: U) {
        self.st.put(key, val);
    }

    pub fn delete(&mut self, key: T) {
        self.st.delete(key);
    }

    pub fn contains(&self, key: T) -> bool {
        self.st.contains(&key)
    }

    pub fn size(&self) -> usize {
        self.st.size()
    }

    pub fn isEmpty(&self) -> bool {
        self.st.size() == 0
    }
}

fn main() {
}