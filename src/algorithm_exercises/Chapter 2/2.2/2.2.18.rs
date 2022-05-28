#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod utils {
    pub mod LinkedList;
}

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p286 - Exercises

// 2.2.18

// Shuffling a linked list
// Divide and conquer to shuffle in O(n lg n) time, and O(lg) space

// Wtf is shuffling?
// If it's a minor variation on merge sort then you get the O(n lg n) time, idk about O(lg) space

// Bottom-up shuffle?
// So shuffle is mixing up the order, swapping around elements
// So [x, y] becomes [y, x], and [1, 2, 3, 4] becomes [3, 4, 1, 2]
// If you keep 'shuffling' in 2^n groups, there are 'N' swaps per level and 'lg N' levels so O(N lg N) swaps
// You should be able to do this in place, so O(1) space? That feels wrong hmm.
// The way I'm thinking about it, you don't need an extra data structures

// Just low, mid, high in shuffle(), then loop 'remove from high -> insert at low' for [mid..high]
// That is an in-place shuffle

// The nice property of a linked list here, over an array, is that removing and inserting from an arbitrary index is O(1) and not O(n)

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