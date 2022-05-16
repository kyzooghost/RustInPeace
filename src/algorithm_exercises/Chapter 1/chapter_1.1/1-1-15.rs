// Write a static method histogram() that takes an array a[] of int values and
// an integer M as arguments and returns an array of length M whose ith entry is the number
// of times the integer i appeared in the argument array. If the values in a[] are all
// between 0 and M–1, the sum of the values in the returned array should be equal to
// a.length.

use std::io;

fn main() {
  let a = vec![1, 2, 3, 4, 5, 6, 7, 9, 11, 12, 16, 17, 18,  21, 24, 25, 26, 32, 34, 36, 37,38, 39, 40, 45, 46, 47,  53, 56, 61, 63, 65, 66, 67, 68, 69, 70, 79, 81, 83, 85,  86, 87, 89, 93, 94, 95, 96, 97, 98];
  
  const M: i32 = 100;

  println!( "{:?}", histogram(&a, M) );
}

fn histogram(array: &[i32], M: i32) -> Vec<i32> {
  use std::collections::btree_map::BTreeMap;
  let mut number_to_count = BTreeMap::new();
  let mut returned_array = vec![];

  for element in array {
    *number_to_count.entry(element).or_insert(0) += 1;
  }

  // Iterate over tree map, insert count into array
  for (_number, _count) in &number_to_count {
    returned_array.push(*_count);
  }

  // Insert 0 into remainder of array
  for _x in array.len()..M as usize {
    returned_array.push(0)
  }

  assert!(returned_array.len() == M as usize);

  returned_array
}