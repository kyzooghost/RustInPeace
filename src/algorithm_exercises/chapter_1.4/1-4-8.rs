#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.8

// Write a program to determine the number pairs of values in an input file that are equal. 
// If your first try is quadratic, think again and use Arrays.sort() to develop a linearithmic solution.

use std::collections::HashMap;
use factorial::Factorial;

fn main() {
    let vec = vec![1, 2, 2, 4, 4, 4, 6, 6, 8, 8, 9, 11, 13, 15, 17, 14, 18];
    println!("{:?}", findNumEqualValues(vec));
}

fn findNumEqualValues(mut vec: Vec<usize>) -> usize {
    vec.sort_unstable();
    // According to https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort
    // O(n*log(n)) worst case
    // Unstable in that it may reorder equal elements
    // Pattern-defeating quicksort
    // Faster than sort() in most cases

    let n = vec.len();
    let mut count = 0;

    let mut hashmap = HashMap::new();

    for i in 0..n {
        if hashmap.contains_key(&vec[i]) {
            let current_count = hashmap.get(&vec[i]).unwrap();
            hashmap.insert(vec[i], current_count + 1);
        } else {
            hashmap.insert(vec[i], 1);
        }
        // O(n) for this inner loop
    }

    // Iterate through hashmap
    // O(n) worst case (if have one of each number)
    for (_, val) in hashmap.iter() {
        if val >= &2 {
            count = count + val.factorial() / ( 2.factorial() * (val - 2).factorial() )
        }
    }

    count
}