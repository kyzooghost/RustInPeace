#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.12

// Write a program that, given two sorted arrays of N int values, 
// prints all elements that appear in both arrays, in sorted order. 
// The running time of your program should be proportional to N in the worst case.

// Does it matter which one has a lower start? Just start from any?
// Initialise new array, commonElements

// index_one = 0, index_two = 0
// Take array_one[index_one], compare to array_two[index_two]
// If ==, add 1 to both indexes, and push array_one[index_one] to commonElements
// If <, add 1 to current index
// If >, add 1 to other index

// Repeat this loop until index_one == array_one.length - 1 || index_two == array_two.length - 1 
// Then whichever index hasn't reached the end, keep incrementing and comparing it to the end of the other array
// If you ever find a match, push that match and break loop
// If you get to end, break loop

// Worst case - O(2n)

use rand::{thread_rng, Rng};

fn main() {
    // Initialize data structures & variables
    let ARRAY_SIZE = 1000;
    let MAX_NUMBER = 1000;
    let mut rng = thread_rng();
    let mut vec_one: Vec<u32> = (0..ARRAY_SIZE).map(|_| rng.gen_range(0..MAX_NUMBER)).collect(); 
    let mut vec_two: Vec<u32> = (0..ARRAY_SIZE).map(|_| rng.gen_range(0..MAX_NUMBER)).collect(); 
    vec_one.sort();
    vec_two.sort();

    let mut commonElements: Vec<u32> = Vec::new();
    let mut index_one = 0;
    let mut index_two = 0;

    // Start timer here
    use std::time::Instant;
    let now = Instant::now();

    while index_one < vec_one.len() - 1 && index_two < vec_two.len() - 1 {
        if vec_one[index_one] == vec_two[index_two] {
            commonElements.push(vec_one[index_one]);
            index_one = index_one + 1;
            index_two = index_two + 1;
        } else if vec_one[index_one] > vec_two[index_two] {
            index_two = index_two + 1;
        } else {
            index_one = index_one + 1;
        }
    }

    if index_one == vec_one.len() - 1 {
        while index_two < vec_two.len() {
            if vec_one[index_one] == vec_two[index_two] {
                commonElements.push(vec_one[index_one]);
                break;
            } else {
                index_two = index_two + 1;
            }
        }
    } else if index_two == vec_two.len() - 1 {
        while index_one < vec_one.len() {
            if vec_one[index_one] == vec_two[index_two] {
                commonElements.push(vec_one[index_one]);
                break;
            } else {
                index_one = index_one + 1;
            }
        }
    }

    println!("{:?}", commonElements);
    let elapsed1 = now.elapsed();
    println!("Time 1: {:.2?}", elapsed1);

    // Test answer with inefficient but easy to confirm algorithm
    // O(n2) algorithm

    let mut commonElements2: Vec<&u32> = Vec::new();
    for element in vec_one.iter() {
        if vec_two.contains(element) {
            commonElements2.push(element)
        }
    }
    
    println!("{:?}", commonElements2);
    let elapsed2 = now.elapsed();
    println!("Time 2: {:.2?}", elapsed2 - elapsed1);
}