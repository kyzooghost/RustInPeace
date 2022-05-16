#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.25 Throwing two eggs from a building. 

// Consider the previous question, but now suppose you only have two eggs, 
// and your cost model is the number of throws. 
// Devise a strategy to determine F such that the number of throws is at most 2√N, 
// then find a way to reduce the cost to ~c √F. 

// This is analogous to a situation where search hits (egg intact) are much cheaper than misses (egg broken).

// Ok the strategy here is throw from every sqrt(N) and find the floor where it breaks
// i.e. sqrt(N), 2 sqrt(N), etc => sqrt(N) worst case to find this floor & break one egg
// Then linear search between (k-1) * sqrt(N) & k * sqrt(N) - worst case sqrt(N) && and break one egg
// Two parts - 2 * sqrt(N) to find, and break both eggs worst case

// Arithmetic sum - 1 + 2 + 3 + ... + k = k^2 / 2 = ~N
// k ~= sqrt(2N), where k is your # of throws and N is your floor
// Then linear search from (x - 1) to (x) floor = Worst case k throws 
// So ~2k = 2 sqrt (2N)

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