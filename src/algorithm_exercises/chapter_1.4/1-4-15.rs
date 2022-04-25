#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.15

// Faster 3-sum. As a warmup, develop an implementation TwoSumFaster that
// uses a linear algorithm to count the pairs that sum to zero after the array is sorted 
// (instead of the binary-search-based linearithmic algorithm). 
// Then apply a similar idea to develop a quadratic algorithm for the 3-sum problem.

use rand::{thread_rng, Rng};

fn main() {
    // Initialize data structures & variables
    let ARRAY_SIZE = 1000;
    let MAX_NUMBER = 100;
    let mut rng = thread_rng();
    let mut vec: Vec<i32> = (0..ARRAY_SIZE).map(|_| rng.gen_range(-MAX_NUMBER..MAX_NUMBER)).collect();
    vec.sort();

    // println!("VEC - {:?}", vec);

    // Start timer here
    use std::time::Instant;
    let now = Instant::now();

    println!("{:?}", TwoSumFaster(vec.clone()));
    let elapsed1 = now.elapsed();
    println!("Time for TwoSumFaster: {:.2?}", elapsed1);

    println!("{:?}", TwoSum(vec.clone()));
    let elapsed2 = now.elapsed();
    println!("Time for TwoSum: {:.2?}", elapsed2 - elapsed1);

    println!("{:?}", ThreeSumFaster(vec.clone()));
    let elapsed3 = now.elapsed();
    println!("Time for ThreeSumFaster: {:.2?}", elapsed3 - elapsed2);

    println!("{:?}", ThreeSum(vec.clone()));
    let elapsed4 = now.elapsed();
    println!("Time for ThreeSum: {:.2?}", elapsed4 - elapsed3);
}

// Want linear algorithm here
// Array is sorted already
// Count pairs that add to zero
// Walk from start and end of array
// Try start + end, if == 0, this is a pair, add to pairArray
// If > 0, need smaller number, end - 1
// If < 0, start + 1
// Keep doing this until crossover
// Gg, the edge cases annoying here - encountering multiples
fn TwoSumFaster(vec: Vec<i32>) -> u32 {
    let mut pairs = Vec::new();
    let mut start = 0;
    let mut end = vec.len() - 1;
    while start < end {
        if vec[start] == 0 && vec[end] == 0 {
            let mut num_of_zeroes = 1;
            
            while vec[start + num_of_zeroes] == 0 {
                num_of_zeroes = num_of_zeroes + 1;
            }

            // n choose 2
            let combinations = factorial(num_of_zeroes) / (2 * factorial(num_of_zeroes - 2));

            for _ in 0..combinations {
                pairs.push((0, 0));
            }

            start = start + num_of_zeroes;
            end = end - num_of_zeroes;
        } else if vec[start] + vec[end] == 0  {
            let original_pair = (vec[start], vec[end]);
            let mut start_copies = 1;
            let mut end_copies = 1;

            while vec[start+1] == vec[start] {
                start_copies = start_copies + 1;
                start = start + 1;
            }

            while vec[end-1] == vec[end] {
                end_copies = end_copies + 1;
                end = end - 1;
            }

            for _ in 0..(start_copies*end_copies) {
                pairs.push(original_pair);
            }

            start = start + 1;
            end = end - 1;
        } else if vec[start] + vec[end] > 0  {
            end = end - 1;
        } else {
            start = start + 1;
        }
    }

    // println!("{:?}", pairs);
    pairs.len() as u32
}

// Naive O(n2) algorithm
fn TwoSum(vec: Vec<i32>) -> u32 {
    let mut count = 0;
    let mut pairsIndex = Vec::new();
    let mut pairs = Vec::new();

    for (i, _) in vec.iter().enumerate() {
        if i == vec.len() - 1 {break;}
        
        for (index, _) in vec[i+1..vec.len()].iter().enumerate() {
            let j = i + 1 + index;
            if vec[i] + vec[j] == 0 {
                count = count + 1;
                pairs.push((vec[i], vec[j]));
                pairsIndex.push((i, j));
            }
        }
    }

    // println!("{:?}", pairs);

    count
}

// Store every pair adding to zero in a hashmap - O(n2)
// Run through every number again, add to pair, if 0 then you have a triplet - O(n)
fn ThreeSumFaster(vec: Vec<i32>) -> u32 {
    use std::collections::HashMap;
    let mut sumMap = HashMap::new();
    let mut triplets = Vec::new();

    // Collect map of sum => vector of pair indexes
    // O(n2)
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

    // Iterate through vector again - O(n)
    for (i, value) in vec.iter().enumerate() {
        if sumMap.contains_key(& (-value)) {
            // count = count + sumMap.get(& (-value)).unwrap().len();

            for pairIndexes in sumMap.get(& (-value)).unwrap() {
                if i > pairIndexes.1 as usize {
                    count = count + 1;
                    triplets.push((pairIndexes.0, pairIndexes.1, i));
                }
            }
        }
    }

    // println!("THREESUMFASTER - {:?}", triplets);

    count as u32
}

// Naive O(n3) algorithm
fn ThreeSum(vec: Vec<i32>) -> u32 {
    let mut count = 0;
    let mut tripletIndex = Vec::new();
    let mut triplets = Vec::new();

    for (i, _) in vec.iter().enumerate() {
        if i == vec.len() - 1 {break;}
        
        for (index_one, _) in vec[i+1..vec.len()].iter().enumerate() {
            let j = i + 1 + index_one;

            for (index_two, _) in vec[j+1..vec.len()].iter().enumerate() {
                let k = j + 1 + index_two;

                if vec[i] + vec[j] + vec[k]  == 0 {
                    count = count + 1;
                    triplets.push((vec[i], vec[j], vec[k]));
                    tripletIndex.push((i, j, k));
                }
            }
        }
    }

    // println!("THREESUM - {:?}", triplets);

    count
}

pub fn factorial(num: usize) -> usize {
    match num {
        0  => 1,
        _ => (1..num+1).product(),
    }
}