#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.18 

// Local minimum of an array. Write a program that, given an array a[] of N distinct integers, finds a local minimum: 
// an index i such that a[i-1] < a[i] < a[i+1]. Your program should use ~2lg N compares in the worst case.

// Answer: Examine the middle value a[N/2] and its two neighbors a[N/2 - 1] and a[N/2 + 1].
// If a[N/2] is a local minimum, stop; otherwise search in the half with the smaller neighbor.

// Urgh, this algorithm bothers me
// Yes you get O(log N) efficiency, but it misses finding a local minimum occasionally. Where the O(n) solution would not.

use rand::{thread_rng, Rng};

fn main() {
    // Initialize data structures & variables
    let ARRAY_SIZE = 25;
    let MAX_NUMBER = 1000;
    let mut rng = thread_rng();
    let vec: Vec<u32> = (0..ARRAY_SIZE).map(|_| rng.gen_range(0..MAX_NUMBER)).collect();

    println!("{:?}", vec);

    // Start timer here
    use std::time::Instant;
    let now = Instant::now();

    println!("Largest diff - {:?}", getLocalMinimum(vec));
    let elapsed1 = now.elapsed();
    println!("Time for getLocalMinimum: {:.2?}", elapsed1);
}

fn getLocalMinimum(vec: Vec<u32>) -> Option<u32> {
    let mut sectionStart = 0;
    let mut sectionEnd = vec.len() - 1;
    let mut index: usize;

    while sectionEnd >= sectionStart {
        index = (sectionEnd + sectionStart) / 2;

        if index == 0 || index == vec.len() - 1 {return None;}
        
        if vec[index] < vec[index - 1] && vec[index] < vec[index + 1] {
            println!("{:?} {:?} {:?}", vec[index - 1], vec[index], vec[index + 1]);
            return Some(index as u32)
        }

        if vec[index - 1] < vec[index + 1] {
            sectionEnd = index - 1;
        } else {
            sectionStart = index + 1;
        }

        println!("START - {:?}", sectionStart);
        println!("END - {:?}", sectionEnd);
    }

    None
}