#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.22 Binary search with only addition and subtraction. [Mihai Patrascu] 

// Write a program that, given an array of N distinct int values in ascending order, 
// determines whether a given integer is in the array. 
// You may use only additions and subtractions and a constant amount of extra memory. 
// The running time of your program should be proportional to log N in the worst case.

// Using fibonacci sequence
// search range [i, i + Fk]
// Keep Fk, Fk-1 in two variables
// At each step, compute Fk-2 via subtraction, check element i + Fk-2

// Fk is the smallest Fib number over n
// Array already sorted
// Keep closing your search interval by stepping down one Fib

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

    println!("{:?}", binarySearch(&vec, 50));

    let elapsed1 = now.elapsed();
    println!("Time for getLocalMinimum: {:.2?}", elapsed1);
}

fn binarySearch(vec: &Vec<i32>, value: i32) -> Option<usize> {
    let mut fibBeforeN = 0;
    let mut fibN = 1;
    let mut placeholder: usize;

    while fibN < vec.len() {
        placeholder = fibN;
        fibN = fibN + fibBeforeN;
        fibBeforeN = placeholder;
    }

    let mut low = 0;
    let mut high = vec.len() - 1;

    while low <= high {

        // downsize for range appropriate
        // fibBeforeN = 0 is the start of the Fib sequence, can't downsize from there
        while fibBeforeN > 0 && fibN > high - low {
            placeholder = fibBeforeN;
            fibBeforeN = fibN - fibBeforeN;
            fibN = placeholder;
        }
        if vec[low + fibBeforeN] == value {return Some(low + fibBeforeN)}
        else if vec[low + fibBeforeN] > value {
            high = low + fibBeforeN - 1;
        } else {
            low = low + fibBeforeN + 1;
        }
    }

    None
}

// So in array of 20
// Fk = 21
// Say element at 17
// [0, 20], look at 13
// Now low = 14, high = 20
// look at 14 + 5 = 19
// Not there, but below
// [14, 18]
// 14 + 3 = 17

// 1, 1, 2, 3, 5, 8, 13, 21, 34