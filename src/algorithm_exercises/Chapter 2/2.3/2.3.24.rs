#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p306 - Exercises

// 2.3.24

// Samplesort
// Sample size of 2^k - 1
// Sort sample, then arrange to have recursive routine 
//  partition on median of sample
//  move two halves of sample to each subarray

// Take sample with the biggest possible value of 2^k - 1
// In recursive part - partition on median of sample, move two halves of sample 

// use rand::{thread_rng};

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    
    quicksort(&mut vec);
    println!("{:?}", vec);
}

fn quicksort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    if array.len() <= 1 {return}
    let j = partition_sample(array);
    let length = array.len();
    quicksort(&mut array[0..j]);
    quicksort(&mut array[j+1..length]);
}

fn partition_sample<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) -> usize {
    // Find biggest k such as 2^k - 1 < array.len()
    let mut k = 1;
    while 2^(k+1) - 1 < array.len() {k += 1;}

    // Sort first k 
    insertion_sort(&mut array[0..k]);

    // Find median, and swap into pivot position
    let pivot = array[k / 2];
    array.swap(0, k / 2);

    let mut i = 1;
    let mut j = array.len() - 1;

    loop {
        // Scan --> until find array[i] > pivot
        while array[i] <= pivot {
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

fn insertion_sort<T: Copy + PartialOrd + std::fmt::Debug>(vec: &mut [T]) {
    let N = vec.len();

    for i in 0..N {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j-1);
            j = j - 1;
        }
    }        
}