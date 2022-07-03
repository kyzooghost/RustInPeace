#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p480 Exercises

// 3.4.3

// Develop an alternate implementation of SeparateChainingHashST
// that directly uses the linked-list code from SequentialSearchST.

mod utils {pub mod LinkedList;}
use utils::LinkedList::List as LinkedList;

pub struct STNode<T, U> {
    key: T,
    value: U
}

pub struct SequentialSearchST<T, U> {
    size: usize,
    list: LinkedList<STNode<T, U>>,
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug, 
    U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    SequentialSearchST<T, U> {

    pub fn size(&self) -> usize {self.size}

    pub fn isEmpty(&self) -> bool {self.size == 0}

    pub fn new() -> Self {
        SequentialSearchST {
            size: 0,
            list: LinkedList::new()
        }
    }

    pub fn get(&self, key: &T) -> Option<U> {
        if self.isEmpty() {return None}

        // Linear search for key
        let iter = self.list.iter();

        for node in iter {
            // Search hit
            if &node.key == key {return Some(node.value)}
        }
        
        None
    }

    pub fn put(&mut self, key: T, value: U) {
        for node in self.list.iter_mut() {
            // Search hit
            if node.key == key {
                node.value = value;
                return
            }
        }

        // Else search miss, insert at end of linked list
        let new_node = STNode{
            key: key,
            value: value
        };

        self.list.insert_at_head(new_node);
        self.size += 1;
    }

    pub fn contains(&self, key: &T) -> bool {
        match self.get(key) {
            Some(_) => true,
            None => false
        }
    }

    pub fn keys(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let iter = self.list.iter();
        for node in iter {vec.push(node.key);}
        vec
    }

    // pub fn delete(&mut self, key: T) -> Option<T> {
    //     let rank = self.rank(key);

    //     // If match current key, change the value
    //     if rank < self.size && self.list.peek_at_index(rank).unwrap().key == key {
    //         let node = self.list.remove_at_index(rank);
    //         self.size -= 1;
    //         return Some(node.unwrap().key)
    //     }

    //     None
    // }
}


fn main() {
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

    for letter in vec {
        println!("{:?}", hash_letter(letter));
    }

}

fn letter_to_number(letter: &str) -> usize {
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

fn hash_letter(letter: &str) -> usize {
    ( 11 * letter_to_number(letter) ) % 5
}