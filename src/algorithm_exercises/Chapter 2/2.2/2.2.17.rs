#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod utils {
    pub mod LinkedList;
}

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p286 - Exercises

// 2.2.17

// Natural mergesort for linked lists

// Is this linearithmic? Yes, merge sort
// Is this in-place? Yes, Took care with merge() to ensure it's in-place. 
// I am creating two new LinkedList() data structures, but I need a place for the
// 'popped' nodes to live while I merge back
// I could have also kept the LinkedList as is, and just peek_mut and changed the elements
// But then I need to copy the elements (vs popping them), to another data structure

// merge_sort without merge() has no extra data structures, just a few primitive variables

use utils::LinkedList::List as LinkedList;

fn main() {
    // Convert vector to linked list
    let mut list = LinkedList::new();
    let vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    for element in vec {list.insert_at_head(element);}
    merge_sort(&mut list);

    for elem in list.iter() {
        println!("{:?}", elem);
    }

}

fn merge_sort<T: Copy + Ord + std::fmt::Debug>(list: &mut LinkedList<T>) {
    let length = list.size();

    let mut i = 0;
    let mut low = 0;
    let mut mid = 0;
    let mut high = 0;

    loop {
        // Get left half
        while i < length - 1 && list.peek_at_index(i).unwrap() <= list.peek_at_index(i+1).unwrap() {i += 1};
        i += 1;
        mid = i;

        if mid == length {
            if low == 0 {break} // 'subarray' is [0..length] => sorted
            else {
                low = 0;
                i = 0;
                continue; // subarray is [n..length] => start from left again
            }
        }

        // Get right half
        while i < length - 1 && list.peek_at_index(i).unwrap() <= list.peek_at_index(i+1).unwrap() {
            i += 1
        };
        high = i;

        // Merge
        merge(list, low, mid, high);

        // Subarray is [n..length] => start from left again
        if high == length {
            i = 0;
            low = 0;
        } else {low = high + 1}
    }
}

// TAIL >>> HEAD
// Maybe I should just use pop left/right & push left/right, instead of insert_at_head, remove_from_tail language
fn merge<T: Copy + PartialOrd + std::fmt::Debug>(list: &mut LinkedList<T>, low: usize, mid: usize, high: usize) {
    assert!(high < list.size());

    // In Rust tradition, mid is exclusive from left, and inclusive in right [low..mid], [mid..right]
    let size_left = mid - low;
    let size_right = high - mid + 1;

    let mut left: LinkedList<T> = LinkedList::new();
    let mut right: LinkedList<T> = LinkedList::new();

    for _ in 0..size_left {
        left.insert_at_head(list.remove_at_index(low).unwrap());
    }

    for _ in 0..size_right {
        right.insert_at_head(list.remove_at_index(low).unwrap());
    }

    for i in 0..size_left + size_right {
        if left.size() == 0 {
            list.insert_at_index(low + i, right.remove_from_tail().unwrap());
        } else if right.size() == 0 {
            list.insert_at_index(low + i, left.remove_from_tail().unwrap());
        } else if left.peek_at_tail().unwrap() < right.peek_at_tail().unwrap() {
            list.insert_at_index(low + i, left.remove_from_tail().unwrap());
        } else {
            list.insert_at_index(low + i, right.remove_from_tail().unwrap());
        }
    }
}