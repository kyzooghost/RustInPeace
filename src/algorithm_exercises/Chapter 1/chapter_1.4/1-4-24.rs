#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.24 Throwing eggs from a building. 
// Suppose that you have an N-story building and plenty of eggs. 
// Suppose also that an egg is broken if it is thrown off floor F or higher, and unhurt otherwise. 
// First, devise a strategy to determine the value of F such that the number of broken eggs is ~lg N when using ~lg N throws, 
// then find a way to reduce the cost to ~2lg F.

// F < N
// First strategy - binary search basically, keep testing the middle floor and halving your floor range each time

// Second strategy - 2 lg F? 2x binary search for F?
// Start from first floor, keep doubling the floors until you find the floor where it breaks. Lg F worst case to find this
// Then binary search within this floor, and the last floor

use rand::{thread_rng, Rng};

fn main() {
    // Initialize data structures & variables
    let mut rng = thread_rng();

    let ARRAY_SIZE:i32 = 100;
    let MAX_NUMBER:i32 = 100;
    let mut vec: Vec<i32> = (0..ARRAY_SIZE).map(|_| rng.gen_range(0..MAX_NUMBER)).collect();
    vec.sort();

    // Start timer here
    use std::time::Instant;
    let now = Instant::now();

    // println!("{:?}", binarySearch(&vec, 50));

    let elapsed1 = now.elapsed();
    println!("Time for getLocalMinimum: {:.2?}", elapsed1);
}

