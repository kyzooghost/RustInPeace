// cargo run wl.txt filth.txt
// .txt files need to be in root folder, parent to ./sr

// Reads two txt file number lists
// Compares the two number lists, and outputs numbers that are in the second .txt file but not the first

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Read two files as command line arguments, and assign to vectors
    let args: Vec<String> = env::args().collect();

    let mut whitelist = create_vector_from_file(&args[1])
      .expect("Error reading file 1");

    let mut filth = create_vector_from_file(&args[2])       
      .expect("Error reading file 2");

    println!("Successfully read both files");

    // Sort the arrays

    let whitelist_length = whitelist.len() as usize;
    let filth_length = filth.len() as usize;

    quicksort(&mut whitelist, 0, whitelist_length - 1);
    quicksort(&mut filth, 0, filth_length - 1);

    println!("Vectors sorted");

    // Create new vector to store unfound items
    let mut unfound_filth: Vec<i32> = Vec::new();

    // Run binary search on each item of filthy list
    for num in &filth {
      // If not found then push to unfound_filth
      if let None = binary_search(&whitelist, 0, whitelist_length - 1, *num) {
        unfound_filth.push(*num);
      }      
    }

    println!("{:?}", whitelist);
    println!("{:?}", filth);
    println!("{:?}", unfound_filth);
}

fn create_vector_from_file<P>(filename: P) -> io::Result<Vec<i32>> where P: AsRef<Path>, {
  let file = File::open(filename)?;
  let lines = io::BufReader::new(file).lines();

  let mut vec = Vec::<i32>::new();

  for line in lines {
    if let Ok(ip) = line {
      vec.push(ip.parse::<i32>().unwrap());
    }
  }
  
  Ok(vec)
}

fn quicksort(arr:&mut Vec<i32>, left: usize, right: usize) {
  // Base case
  if right <= left {return}

  // Partition
  let j:usize = quicksort_partition(arr, left, right); 
  quicksort(arr, left, j - 1);
  quicksort(arr, j + 1, right);
}

fn quicksort_partition (arr:&mut Vec<i32>, left: usize, right: usize) -> usize {
  let mut i = left + 1;
  let mut j = right;
  let partition_element: i32 = arr[left]; // Arbitrary choice of partition element => Will find final place in this iteration

  // Scan from left until find entry >= partition_element
  // Scan from right until find entry <= partition_element
  // These two items out of place, so swap
  // Ensure no element to left of i is greater than partition element
  // And no element to the right of j is lesser than partition element
  loop {
    // Scan from left, until entry >= partition_element
    // Test if arr[i] >= partition_element, if not move one right 
    while arr[i] <= partition_element {
      if i == right {break}
      i += 1;
    }

    // Scan from right, until entry <= partition_element
    while partition_element < arr[j] {
      if j == left {break}
      j -= 1;
    }

    if i >= j {break} // Break loop when crossover

    swap_array_elements(arr, i, j);
  }

  // When i and j meet, swap partition element with rightmost end of left subarray
  swap_array_elements(arr, left, j);

  // Now partitioned into [..i - <= partition_element], [partition_element], [j.. - >= partition_element]

  j // Return index where partition element put into
}

fn swap_array_elements(arr:&mut Vec<i32>, index_one: usize, index_two: usize) -> &mut Vec<i32> {
  let temp = arr[index_one];
  arr[index_one] = arr[index_two];
  arr[index_two] = temp;
  arr
}

fn binary_search(_array: & Vec<i32>, _start: usize, _end:usize, _x:i32) -> Option<usize> {
  if _start > _end {return None}

  let mid = (_start + _end) / 2;

  if _x < _array[mid as usize] {
    return binary_search(_array, _start, mid - 1, _x)
  }

  else if _x > _array[mid as usize] {
    return binary_search(_array, mid + 1, _end, _x)
  }

  else {return Some(mid)}

}