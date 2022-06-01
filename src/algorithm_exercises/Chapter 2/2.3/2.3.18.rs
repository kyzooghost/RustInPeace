#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p305 - Exercises

// 2.3.18

// Median-of-three partitioning
// Use median of small sample of items taken from subarray, partition on median
// Sample items as sentinels at the end of the array and remove both array bound tests

fn main() {
    // use rand::{thread_rng, seq::SliceRandom};
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    
    quicksort(&mut vec);
    println!("{:?}", vec);

    println!("{:?}", quickselect(&mut vec, 7));
}

fn quicksort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {

    if array.len() <= 1 {
        return
    }

    let j = partition_with_median(array);
    let length = array.len();
    quicksort(&mut array[0..j]);
    quicksort(&mut array[j+1..length]);
}

fn partition_with_median<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) -> usize {
    // Can only do median-of-three if there are >= 3 elements
    let mut pivot = array[0];

    if array.len() >= 3 {
        pivot = quickselect(&mut array[0..2], 1); // choose median from first three elements in array
        // Find element of pivot

        if pivot == array[1] {array.swap(0, 1)}
        else if pivot == array[2] {array.swap(0,2)}
    }

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

// I want an O(N) algorithm to find the median
fn quickselect<T: Copy + Ord + std::fmt::Debug>(array: &mut [T], index: usize) -> T {
    let length = array.len();
    if length == 1 {return array[0]}

    let j = partition(array);

    if index == j {
        return array[j]
    } else if index < j {
        quickselect(&mut array[0..j], index)
    } else {
        quickselect(&mut array[j+1..length], index - j - 1)
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