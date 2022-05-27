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

fn main() {
    // Convert vector to linked list
    use utils::LinkedList::List as LinkedList;
    let mut list = LinkedList::new();
    let vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    for element in vec {list.insert_at_head(element);}
    println!("{:?}", list.peek_at_tail());
    println!("{:?}", list.peek_at_head());
    println!("{:?}", list.peek_at_index(0));
    println!("{:?}", list.peek_at_index(1));
    println!("{:?}", list.peek_at_index(2));
    println!("{:?}", list.peek_at_index(3));
}

fn merge_sort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    let length = array.len();

    let mut i = 0;
    let mut left: &[T];
    let mut right: &[T];
    let mut low = 0;
    let mut mid = 0;
    let mut high = 0;

    loop {
        // Get left half
        while i < length - 1 && array[i] <= array[i + 1] {i += 1};
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

        left = &array[low..mid];

        // Get right half
        while i < length - 1 && array[i] <= array[i + 1] {i += 1};
        i += 1;
        high = i;
        right = &array[mid..high];

        // Merge
        let mut temp: Vec<T> = array.to_vec();
        merge(left, right, &mut temp[low..high]);
        array[low..high].copy_from_slice(&temp[low..high]);

        // Subarray is [n..length] => start from left again
        if high == length {
            i = 0;
            low = 0;
        } else {low = high + 1}
    }
}

fn merge<T: Copy + PartialOrd + std::fmt::Debug>(left: &[T], right: &[T], array: &mut [T]) {
    assert_eq!(left.len() + right.len(), array.len());
    let mut i = 0;
    let mut j = 0;
    let length = array.len();

    for k in 0..length {
        if i >= left.len() {
            array[k] = right[j];
            j += 1;
        } else if j >= right.len() {
            array[k] = left[i];
            i += 1;
        } else if left[i] < right[j] {
            array[k] = left[i];
            i += 1;
        } else {
            array[k] = right[j];
            j += 1;
        }
    }
}