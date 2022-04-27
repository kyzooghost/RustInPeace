#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.17 

// Farthest pair (in one dimension). 
// Write a program that, given an array a[] of N double values, 
// finds a farthest pair : 
// two values whose difference is no smaller than the the difference of any other pair 
// (in absolute value). 
// The running time of your program should be linear in the worst case.

// Linear time, farthest
// Can't use sort, that's O(n log n) already
// Some sort of iteration over array

// Find your largest element - O(n)
// Find your smallest element - O(n)
// This is your farthest pair

use rand::{thread_rng, Rng};

fn main() {
    // Initialize data structures & variables
    let ARRAY_SIZE = 800;
    let MAX_NUMBER = 1000.00;
    let mut rng = thread_rng();
    let vec: Vec<f32> = (0..ARRAY_SIZE).map(|_| rng.gen_range(-MAX_NUMBER..MAX_NUMBER)).collect();

    println!("{:?}", vec);

    // Start timer here
    use std::time::Instant;
    let now = Instant::now();

    println!("Largest diff - {:?}", getFathestPair(vec));
    let elapsed1 = now.elapsed();
    println!("Time for getFathestPair: {:.2?}", elapsed1);
}

fn getFathestPair(vec: Vec<f32>) -> f32 {
    let mut min = vec[0];
    let mut minIndex = 0;
    let mut max = vec[0];
    let mut maxIndex = 0;

    for (i, element) in vec.iter().enumerate() {
        if element < &min {
            min = *element;
            minIndex = i;
        }
    }

    for (i, element) in vec.iter().enumerate() {
        if element > &max {
            max = *element;
            maxIndex = i;
        }
    }

    println!("MIN - {:?}", min);
    println!("MAX - {:?}", max);

    println!("MIN_INDEX - {:?}", minIndex);
    println!("MAX_INDEX - {:?}", maxIndex);

    max - min
}