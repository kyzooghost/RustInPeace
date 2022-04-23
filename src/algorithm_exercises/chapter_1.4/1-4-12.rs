#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.12

// Write a program that, given two sorted arrays of N int values, prints all elements
// that appear in both arrays, in sorted order. The running time of your program
// should be proportional to N in the worst case.

// Two sorted arrays of N, print all elements in both, in sorted order
// Linear time worst case - for-loop

// Naive solution, for each element in vec1, search for 
// For loop -> Binary search -> O(N log N)

use rand::Rng;
use std::collections::HashMap;

fn main() {
  // Create two sorted vectors of N int values
  let N = 100;
  let max = 1000;
  let mut rng = rand::thread_rng();
  let mut vec1 = Vec::new();
  let mut vec2 = Vec::new();

  for i in 0..N {
    vec1.push(rng.gen_range(0..max));
    vec2.push(rng.gen_range(0..max));
  }

  vec1.sort();
  vec2.sort();

  //
  let mut map = HashMap::new();

  // ~n
  for i in vec1 {
    let counter = map.entry(i).or_insert(0);
    *counter += 1;
  }

  // ~n
  for j in vec2 {
    let counter = map.entry(j).or_insert(0);
    *counter += 1;
  }

  let mut vec3 = Vec::new();

  // ~2n
  for (key, val) in map.iter() {
    if val > &1 {
      vec3.push(key)
    }
  }

  println!("{:?}", vec3);
  
}