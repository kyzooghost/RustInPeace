#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.14

// 4-sum. Develop an algorithm for the 4-sum problem.
// Given an array, find an algorithm to count the number of combinations of four elements summing to 4

use rand::{thread_rng, Rng};

fn main() {
    // Initialize data structures & variables
    let ARRAY_SIZE = 200;
    let MAX_NUMBER = 1000;
    let mut rng = thread_rng();
    let vec: Vec<i32> = (0..ARRAY_SIZE).map(|_| rng.gen_range(-MAX_NUMBER..MAX_NUMBER)).collect();

    // Start timer here
    use std::time::Instant;
    let now = Instant::now();

    println!("{:?}", FourSumSlow(vec.clone()));

    let elapsed1 = now.elapsed();
    println!("Time for FourSumSlow: {:.2?}", elapsed1);

    println!("{:?}", FourSumFast(vec));

    let elapsed2 = now.elapsed();
    println!("Time for FourSumFast: {:.2?}", elapsed2 - elapsed1);
}

fn FourSumFast(vec: Vec<i32>) -> u32 {
    use std::collections::HashMap;
    let mut sumMap = HashMap::new();
    
    // Create hashmap, key = every possible sum of two, value = vector of tuple of indexes of matching pairs
    // This part - O(n^2)
    for (i, _) in vec.iter().enumerate() {        
        if i == vec.len() - 1 {break;}

        for (index, _) in vec[i+1..vec.len()].iter().enumerate() {
            let j = i + 1 + index;
            let sum = vec[i] + vec[j];

            if !sumMap.contains_key(&sum) {
                sumMap.insert(sum, Vec::new());
            }

            let current_vec = sumMap.get_mut(&sum).unwrap();
            current_vec.push( (i as i32, j as i32) );
        }

    }

    let mut count = 0;

    // n (n-1) sums => Best case O(n2), and this appears to be the aggregated case
    // Iterate through each key in sumMap
    for (sum, pairVector) in sumMap.clone() {
        // If has complement key (sum)
        if sumMap.contains_key(&(sum * -1)) {
            let pairComplementVector = sumMap.get(&(sum * -1)).unwrap();

            // Iterate through each pair in pairVector - 
            // Iterate through each pair in pairComplementVector
            for element in pairVector {
                for elementComplement in pairComplementVector {
                    if element.1 < elementComplement.0 {
                        count = count + 1;
                    }
                }
            }
        }
    }

    count
}

fn FourSumSlow(mut vec: Vec<i32>) -> u32 {
    let mut count = 0;
    vec.sort(); // O (n log(n))
    let mut vec_group: Vec<(i32, i32, i32, i32)> = Vec::new();

    // Three-layer nested for-loop + binary search => O (n^3 log(n))
    for (i, _) in vec.iter().enumerate() {
        if i == vec.len() - 1 {break;}
        
        for (index_one, _) in vec[i+1..vec.len()].iter().enumerate() {
            let j = i + 1 + index_one;

            for (index_two, _) in vec[j+1..vec.len()].iter().enumerate() {
                let k = j + 1 + index_two;

                match vec.binary_search(& (- vec[i] - vec[j] - vec[k])) {
                    Err(_) => {},
                    Ok(index_found) => {
                        if index_found > k {
                            count = count + 1;
                            vec_group.push((vec[i], vec[j], vec[k], vec[index_found]));
                        }
                    }
                }
            }
        }

    }

    count
}