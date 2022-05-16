#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.20 Bitonic search. 

// An array is bitonic if it is comprised of an increasing sequence of integers 
// followed immediately by a decreasing sequence of integers. 
// Write a program that, given a bitonic array of N distinct int values, 
// determines whether a given integer is in the array. 
// Your program should use ~3lg N compares in the worst case.

// Increasing then decreasing?
// Binary search would do this in lg N, why do you need 3 lg N?
// Oh right, binary search needs a sorted array ok

// Look at middle, if equal return, 

// Find tipping point or max value - O(log N)
// Binary search left half - O (log N)
// Binary search right half - O (log N)

// Return none if low > high
// Recursive
// Cases - i.) middle is within rising ii.) middle is within declining
// If in rising - bigger than value, smaller than value
// If in declining - bigger than value, smaller than value

use rand::{thread_rng, Rng};

fn main() {
    // Initialize data structures & variables
    let mut rng = thread_rng();

    let MATRIX_SIZE:i32 = 10;
    let TIPPING_POINT_INDEX = rng.gen_range(1..MATRIX_SIZE-1);
    let MAX_JUMP:i32 = 10;

    let mut vec = Vec::new();
    vec.push(rng.gen_range(0..MAX_JUMP));

    for i in 0..TIPPING_POINT_INDEX {
        vec.push(vec[i as usize] + rng.gen_range(1..MAX_JUMP))
    }

    for i in TIPPING_POINT_INDEX..MATRIX_SIZE-1 {
        vec.push(vec[i as usize] - rng.gen_range(1..MAX_JUMP))
    }

    let vec_len = vec.len();

    // Start timer here
    use std::time::Instant;
    let now = Instant::now();

    println!("{:?}", vec);
    println!("{:?}", bitonicSearch(&vec, 20, 0, vec_len - 1));

    let elapsed1 = now.elapsed();
    println!("Time for getLocalMinimum: {:.2?}", elapsed1);
}

fn bitonicSearch(vec: &Vec<i32>, value: i32, low: usize, high: usize) -> Option<usize> {
    let tippingPointIndex = findTippingPointIndex(&vec);
    println!("tippingPointIndex - {:?}", tippingPointIndex);
    println!("tippingPoint - {:?}", vec[tippingPointIndex]);
    if value == vec[tippingPointIndex] {return Some(tippingPointIndex)}

    // Binary search left of tipping point, then to the right
    match ascendingBinarySearch(vec, value, low, tippingPointIndex) {
        Some(index) => return Some(index),
        None => match descendingBinarySearch(vec, value, tippingPointIndex, high) {
            Some(index) => return Some(index),
            None => return None
        }
    }
}

fn findTippingPointIndex(vec: &Vec<i32>) -> usize {
    let mut low = 0;
    let mut high = vec.len();
    let mut indexToReturn = (low + high) / 2;

    while vec[indexToReturn] < vec[indexToReturn-1] || vec[indexToReturn] < vec[indexToReturn+1] {
        // if in ascending, must be to the right
        if vec[indexToReturn-1] < vec[indexToReturn] && vec[indexToReturn] < vec[indexToReturn+1]  {
            low = indexToReturn;
            indexToReturn = (low + high) / 2;
        // if in descending, must be to the left
        } else if vec[indexToReturn-1] > vec[indexToReturn] && vec[indexToReturn] > vec[indexToReturn+1] {
            high = indexToReturn;
            indexToReturn = (low + high) / 2;
        }
    }

    indexToReturn
}

// Regular binary search
fn ascendingBinarySearch(vec: &Vec<i32>, value: i32, low: usize, high: usize) -> Option<usize> {
    println!("A low - {:?}, A high - {:?}", low, high);
    
    // Base case
    if low > high {return None}

    let middle = (high + low) / 2;
    if value == vec[middle] {return Some(middle)}

    if value > vec[middle] {
        return ascendingBinarySearch(vec, value, middle + 1, high)
    } else if value < vec[middle] {
        return ascendingBinarySearch(vec, value, low, middle - 1)
    }

    None
}

// Regular binary search
fn descendingBinarySearch(vec: &Vec<i32>, value: i32, low: usize, high: usize) -> Option<usize> {
    println!("D low - {:?}, D high - {:?}", low, high);

    // Base case
    if low > high {return None}

    let middle = (high + low) / 2;
    if value == vec[middle] {return Some(middle)}

    if value > vec[middle] {
        return descendingBinarySearch(vec, value, low, middle - 1)
    } else if value < vec[middle] {
        return descendingBinarySearch(vec, value, middle + 1, high)
    }

    None
}