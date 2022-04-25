#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.16

// Closest pair (in one dimension). 
// Write a program that, given an array a[] of N double values, 
// finds a closest pair : two values whose difference is no greater than 
// the the difference of any other pair (in absolute value). 
// The running time of your program should be linearithmic in the worst case.

use rand::{thread_rng, Rng};

fn main() {
    // Initialize data structures & variables
    let ARRAY_SIZE = 800;
    let MAX_NUMBER = 100.00;
    let mut rng = thread_rng();
    let mut vec: Vec<f32> = (0..ARRAY_SIZE).map(|_| rng.gen_range(-MAX_NUMBER..MAX_NUMBER)).collect();

    // Start timer here
    use std::time::Instant;
    let now = Instant::now();

    println!("Smallest diff - {:?}", getClosestPair(vec.clone()));
    let elapsed1 = now.elapsed();
    println!("Time for getClosestPair: {:.2?}", elapsed1);
}

// sort - O(n log n)
// Walk amongst each pair O(n)
// Store closestPair
fn getClosestPair(mut vec: Vec<f32>) -> f32 {
    // O(n log n)
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("{:?}", vec);
    let mut closestPairIndexes = (0, 1);
    let mut closestPair = (vec[0], vec[1]);
    let mut smallestDifference = vec[1] - vec[0];

    // Iterate through vec - O(n)
    for (i, _) in vec.iter().enumerate() {
        if i + 2 == vec.len() {
            break;
        }

        println!("{:?}", vec[i + 1] - vec[i]);

        if vec[i + 1] - vec[i] < smallestDifference {
            closestPairIndexes = (i, i + 1);
            closestPair = (vec[i], vec[i+1]);
            smallestDifference = vec[i+1] - vec[i];
        }
    }

    println!("{:?}", closestPair);
    smallestDifference as f32
}