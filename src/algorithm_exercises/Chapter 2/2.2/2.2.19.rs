#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p286 - Exercises

// 2.2.19

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    println!("{:?}", merge_sort(&mut vec));
    // println!("{:?}", vec);
}

fn merge_sort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) -> usize {
	let mut count = 0;
    let length = array.len();
	if length <= 1 {return count;}

	count += merge_sort(&mut array[0..length/2]);
	count += merge_sort(&mut array[length/2..length]);
 
	let mut y: Vec<T> = array.to_vec();
	count += merge(&array[0..length/2], &array[length/2..length], &mut y[..]);
	array.copy_from_slice(&y);

    count
}

// Ah yes, you do need a mutable ref in Rust, to get over 'moving'
fn merge<T: Copy + PartialOrd + std::fmt::Debug>(left: &[T], right: &[T], array: &mut [T]) -> usize {
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
        } else if left[i] < right[j] {
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