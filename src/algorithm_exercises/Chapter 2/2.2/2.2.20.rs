#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p286 - Exercises

// 2.2.20
// Indirect merge sort - does not rearrange array, but returns perm[] that is the rearranged array

fn main() {
    let vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    println!("{:?}", merge_sort(&vec));
    // println!("{:?}", vec);
}

fn merge_sort<T: Copy + Ord + std::fmt::Debug>(array: &Vec<T>) -> Vec<T> {
    let length = array.len();
    let mut array_clone = array.to_vec();

    let mut size = 1;
    while size < length {
        let mut low = 0;
        while low < length - size {
            let mut y: Vec<T> = array_clone.to_vec();
            merge(&array_clone[low..low + size], &array_clone[low + size..std::cmp::min(low + size + size, length)], &mut y[low..std::cmp::min(low + size + size, length)]);
            array_clone.copy_from_slice(&y);
            low = low + size + size;
        }
        size += size;
    }

    array_clone
}

// Ah yes, you do need a mutable ref in Rust, to get over 'moving'
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