#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p355 Exercises

// 2.5.19

// Write a program KendallTau.java that computes the Kendall tau distance between two permutations in linearithmic time.

// Two permutations
// Last time we piggy backed off merge sort to find the Kendall tau distance b
// In the merge, each time the element from the left half > element from right half
// That is an inversion

// The question becomes, how many swaps do we need to go from one permutation to another?
// It's basically a sort question, you treat one permutation as the 'sorted permutation' and sort the 
// other permutation into it
// You just change the compareTo function here

use std::collections::HashMap; 

fn main() {
    let array1 = vec![0, 3, 1, 6, 2, 5, 4];
    let array2 = vec![1, 0, 3, 6, 4, 2, 5];
    println!("{:?}", find_kendall_tau(&array1, &array2));
}

fn find_kendall_tau<T: Copy + Ord + std::fmt::Debug + std::hash::Hash>(array_1: &Vec<T>, array_2: &Vec<T>) -> usize {
    let mut ranking_hashmap = HashMap::new();
    for (i, element) in array_2.iter().enumerate() {
        ranking_hashmap.insert(element, i);
    }

    merge_sort(&mut array_1.clone(), &ranking_hashmap)   
}

fn merge_sort<T: Copy + Ord + std::fmt::Debug + std::cmp::Eq + std::hash::Hash>(array: &mut [T], ranking_hashmap: &HashMap<&T, usize>) -> usize {
	let mut count = 0;
    let length = array.len();
	if length <= 1 {return count;}

	count += merge_sort(&mut array[0..length/2], ranking_hashmap);
	count += merge_sort(&mut array[length/2..length], ranking_hashmap);
 
	let mut y: Vec<T> = array.to_vec();
	count += merge(&array[0..length/2], &array[length/2..length], &mut y[..], ranking_hashmap);
	array.copy_from_slice(&y);

    count
}

fn merge<T: Copy + PartialOrd + std::fmt::Debug + std::hash::Hash + std::cmp::Eq>(left: &[T], right: &[T], array: &mut [T], ranking_hashmap: &HashMap<&T, usize>) -> usize {
    assert_eq!(left.len() + right.len(), array.len());
    let mut count = 0;
    let mut i = 0;
    let mut j = 0;
    let length = array.len();

    for k in 0..length {
        if i >= left.len() {
            array[k] = right[j];
            j += 1;
        } else if j >= right.len() {
            array[k] = left[i];
            i += 1;
        // } else if left[i] < right[j] {
        } else if lessThan(&left[i], &right[j], ranking_hashmap) {
            array[k] = left[i];
            i += 1;
        } else {
            // left[i] > right[j]
            // All of left[i..] is > right[j]
            // Each (left[i..], right[j]) is an inversion
            count += left[i..].len();
            array[k] = right[j];
            j += 1;
        }
    }

    count
}

fn lessThan<T: Copy + PartialOrd + std::fmt::Debug + std::hash::Hash + std::cmp::Eq>(left: &T, right: &T, ranking_hashmap: &HashMap<&T, usize>) -> bool {
    let left_rank = ranking_hashmap.get(left).unwrap();
    let right_rank = ranking_hashmap.get(right).unwrap();

    if left_rank < right_rank {return true}
    false
}