#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.4 - Develop a table like the one on page 181 for TwoSum.
use num_bigint::{ToBigInt};

fn main() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -1, -3, -9, -45, -65, -98, -2, -4, -5, -6];
    println!("{:?}", TwoSum(vector));
}

fn TwoSum(vector: Vec<i32>) -> u32 {
    let N = vector.len(); // Constant
    let mut count = 0; // Constant

    for i in 0..N { // N
        for j in i..N { // (N - 1) / 2
                if vector[i] + vector[j] == 0 {count = count + 1;} // Inner-loop
        }
    }

    count // Constant
}

// Tilde approximation ~1/2 N^2
// Order of growth N^2