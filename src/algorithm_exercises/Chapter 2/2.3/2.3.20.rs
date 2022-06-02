#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p305 - Exercises

// 2.3.20

// Non-recursive quicksort

// God fuck this is such a pain in the ass in Rust, ownership rules while
// mutating the same data structure in a while loop urgh

// ~50 lines iterative vs ~5 lines recursive

use rand::{thread_rng};

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    
    quicksort_iterative(&mut vec);
    println!("{:?}", vec);
}

// Stack = Arrays to be partitioned
fn quicksort_iterative<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    let length = array.len();
    if length <= 1 {return}
    let mut subarray_stack = Vec::new(); // Stack of subarray to partition
    let mut index_stack: Vec<(usize, usize)> = Vec::new(); // Stack to keep track of subarray index

    // Cannot push original array onto stack, due to ownership rules
    // Clone array => Push copy onto stack
    subarray_stack.push(array.to_vec());
    index_stack.push((0, length));

    // Loop of pop stack => partition it => merge back into main 
    // => push two subarrays onto stack (bigger subarray first)
    while subarray_stack.len() > 0 {
        let mut subarray = subarray_stack.pop().unwrap();
        let subarray_indexes = index_stack.pop().unwrap();
        let start = subarray_indexes.0;
        let end = subarray_indexes.1;

        let j = partition(&mut subarray);
        array[start..end].copy_from_slice(&subarray);

        let left_subarray_after_partition = array[start..start+j].to_vec();
        let right_subarray_after_partition = array[start+j+1..end].to_vec();
        let left_size = left_subarray_after_partition.len();
        let right_size = right_subarray_after_partition.len();

        // Only push subarray if subarray.len() > 1
        // If first subarray bigger, push first subarray first
        if left_size > right_size {
            if left_size > 1 {
                subarray_stack.push(left_subarray_after_partition);
                index_stack.push((start, start+j));
            }
            if right_size > 1 {
                subarray_stack.push(right_subarray_after_partition);
                index_stack.push((start+j+1, end));
            }
        } else {
            if right_size > 1 {
                subarray_stack.push(right_subarray_after_partition);
                index_stack.push((start+j+1, end));
            }
            if left_size > 1 {
                subarray_stack.push(left_subarray_after_partition);
                index_stack.push((start, start+j));
            }
        }
    }
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