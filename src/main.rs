#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod utils {
    pub mod LinkedList;
}

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p286 - Exercises

// 2.2.19

use utils::LinkedList::List as LinkedList;
use std::cmp::min as min;

fn main() {
    // Convert vector to linked list
    let mut list = LinkedList::new();
    let vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    for element in vec {list.insert_at_head(element);}
    merge_shuffle(&mut list);

    for elem in list.iter() {
        println!("{:?}", elem);
    }

}

fn merge_shuffle<T: Copy + Ord + std::fmt::Debug>(list: &mut LinkedList<T>) {
    let length = list.size();

    let mut size = 1;
    while size < length {
        let mut low = 0;
        while low < length - size {
            shuffle(list, low, low + size, min(low + size + size, length - 1));
            low = low + size + size;
        }
        size += size;
    }

}

// TAIL >>> HEAD
// Maybe I should just use pop left/right & push left/right, instead of insert_at_head, remove_from_tail language
fn shuffle<T: Copy + PartialOrd + std::fmt::Debug>(list: &mut LinkedList<T>, low: usize, mid: usize, high: usize) {
    assert!(high < list.size());

    // In Rust tradition, mid is exclusive from left, and inclusive in right [low..mid], [mid..right]
    let size_left = mid - low;
    let size_right = high - mid + 1;

    for _ in 0..size_right {
        let elem = list.remove_at_index(high).unwrap();
        list.insert_at_index(low, elem);
    }
}