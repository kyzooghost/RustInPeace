#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p305 - Exercises

// 2.3.20

// Non-recursive quicksort

use rand::{thread_rng};

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    
    quicksort(&mut vec);
    println!("{:?}", vec);

    // for i in 0..vec.len() {println!("{:?}", quickselect(&vec, i));}
}
fn quicksort_iterative<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    let stack = Vec::new();
    if array.len() <= 1 {return}
    let j = partition(array);
    let length = array.len();
    quicksort(&mut array[0..j]);
    quicksort(&mut array[j+1..length]);
}

fn quicksort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    if array.len() <= 1 {return}
    let j = partition(array);
    let length = array.len();
    quicksort(&mut array[0..j]);
    quicksort(&mut array[j+1..length]);
}

// I want an O(N) algorithm to find the median
fn quickselect<T: Copy + Ord + std::fmt::Debug>(array: &[T], index: usize) -> T {
    let length = array.len();
    if length == 1 {return array[0]}

    let j = partition(&mut array.to_vec());

    if index == j {
        return array[j]
    } else if index < j {
        quickselect(&array[0..j], index)
    } else {
        quickselect(&array[j+1..length], index - j - 1)
    }
}

fn partition<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) -> usize {
    // Can only do median-of-three if there are >= 3 elements
    let pivot = array[0];
    let mut i = 1;
    let mut j = array.len() - 1;

    loop {
        // Scan --> until find array[i] > pivot
        while array[i] < pivot {
            if i == array.len() - 1 {break;}
            i += 1;
        }

        // Scan <-- until find array[j] < pivot>
        while pivot < array[j] {
            j -= 1;
        }

        if i >= j {break;}

        array.swap(i, j);
    }

    array.swap(0, j);

    j
}