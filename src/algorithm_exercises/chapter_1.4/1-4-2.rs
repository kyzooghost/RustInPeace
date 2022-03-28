#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.2 - Modify ThreeSum to work properly even when the int values are so large that adding two of them might cause overflow.

use num_bigint::{ToBigInt};

fn main() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -1, -3, -9, -45, -65, -98, -2, -4, -5, -6];
    println!("{:?}", Threesum(vector));
}

fn Threesum(vector: Vec<i32>) -> u32 {
    let N = vector.len();
    let mut count = 0;

    for i in 0..N {
        for j in i..N {
            for k in j..N {
                // if vector[i] + vector[j] + vector[k] == 0 {count = count + 1;}
                let a = vector[i].to_bigint().unwrap();
                let b = vector[j].to_bigint().unwrap();
                let c = vector[k].to_bigint().unwrap();
                if a + b + c == 0.to_bigint().unwrap() {count = count + 1}
            }
        }
    }

    count
}