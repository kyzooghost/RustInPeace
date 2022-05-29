#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p286 - Exercises

// 2.2.21

// Triplicates. 
// Given three lists of N names each, devise a linearithmic algorithm to determine.
// If there is any name common to all three lists, and if so, return the first such name.

// Doesn't say if the lists are sorted or not yet
// Let's use merge sort to sort each list, so ~3 N lg (N) time cost for this
// Then collect all elements into a hash map "element => count" ~N

// Iterate through hash map entry, and if count >= 2, (Outer loop ~N)
// Binary search through each of the three lists, and if get >=2 positive results then return the name (Inner loop ~3 lg N)

// ~6 N lg N + N 
// O (N lg N)


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