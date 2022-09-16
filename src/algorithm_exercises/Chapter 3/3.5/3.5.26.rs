#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p522

// 3.5.26 - LRU Cache

// Create a data structure that supports the following operations: access and remove. The access operation inserts the item onto the data structure if it’s not already present. The remove operation deletes and returns the item that was least recently accessed. 

// Hint : Maintain the items in order of access in a doubly linked list, along with pointers to the first and last nodes. Use a symbol table with keys = items, values = location in linked list. When you access an element, delete it from the linked list and reinsert it at the beginning. When you remove an element, delete it from the end and remove it from the symbol table.

// Access - insert item into data structure if not present
// Remove - Delete and remove item least recently accessed
// Store item in doubly linked list, pointers to first and last nodes
// Symbol table, key = items, values = index in linked list

use std::hash::{Hash};
mod utils {pub mod LinearProbingHashTable; pub mod LinkedList;}
use utils::LinearProbingHashTable::LinearProbingHashTable as ST;
use utils::LinkedList::LinkedList as LinkedList;

pub struct LRUCache<T> {
    st: ST<T, usize>,
    list: LinkedList<T>
}

// TAIL (0, least recently accessed item) >> HEAD (most recently accessed item)

impl<T: Copy + Clone + PartialEq + PartialOrd + std::fmt::Debug + Hash> LRUCache<T> {
    pub fn new() -> Self {
        let st = ST::new(997);
        let list = LinkedList::new();
        LRUCache {st: st, list: list}
    }

    pub fn access(&mut self, item: T) {
        if self.st.contains(&item) {
            // Remove item from original position in list
            let index = self.st.get(&item).unwrap();
            self.list.remove_at_index(index);

            // Iterate from original position to head, update symbol table with new indexes
            for i in index..self.list.size() {
                let element = self.list.peek_at_index(i).unwrap();
                self.st.put(*element, i);
            }
        }

        // Insert element at head
        self.list.insert_at_head(item);
        self.st.put(item, self.list.size() - 1);
    }

    pub fn remove(&mut self) -> T {
        let item = self.list.remove_from_tail().unwrap();
        self.st.delete(item);

        // Iterate through list, update symbol table with new indexes
        for i in 0..self.list.size() {
            let element = self.list.peek_at_index(i).unwrap();
            self.st.put(*element, i);
        }

        item
    }
}

fn main() {
    let mut cache = LRUCache::new();
    cache.access(10);
    cache.access(9);
    cache.access(8);
    println!("{:?}", cache.st.get(&10));
    println!("{:?}", cache.st.get(&9));
    println!("{:?}", cache.st.get(&8));

    println!("---ACCESS---");
    cache.access(10);
    println!("{:?}", cache.st.get(&9));
    println!("{:?}", cache.st.get(&8));
    println!("{:?}", cache.st.get(&10));

    println!("---REMOVE---");
    cache.remove();
    println!("{:?}", cache.st.get(&9));
    println!("{:?}", cache.st.get(&8));
    println!("{:?}", cache.st.get(&10));
}