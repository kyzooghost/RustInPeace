#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p305 - Exercises

// 2.3.18

// Median-of-five partitioning
// Partition on median of random sample of five items from subarray

use rand::{thread_rng};

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    
    quicksort(&mut vec);
    println!("{:?}", vec);

    // for i in 0..vec.len() {println!("{:?}", quickselect(&vec, i));}
}

fn quicksort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    if array.len() <= 1 {return}
    let j = partition_with_median(array);
    let length = array.len();
    quicksort(&mut array[0..j]);
    quicksort(&mut array[j+1..length]);
}

fn partition_with_median<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) -> usize {
    
    // Can only do median-of-five if there are >= 5 elements
    let mut pivot = array[0];

    if array.len() >= 5 {
        // Get your five random elements
        let mut rng = thread_rng();
        let random_indexes = rand::seq::index::sample(&mut rng, array.len(), 5).into_vec();
        let mut random_elements = Vec::new();
        for index in random_indexes.clone() {
            random_elements.push(array[index]);
        }

        // Get your median
        pivot = quickselect(&random_elements, 2);

        // Swap median into start of array
        for i in 0..5 {
            if pivot == random_elements[i] {array.swap(0, random_indexes[i])}
        }
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