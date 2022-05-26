#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p285 - Exercises

// 2.2.11

// merge() with
// i.) Insertion sort if N <= 16
// ii.) Skip merge if array already in order
// iii.) Switch arguments in recursive code?

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    let mut aux = vec.clone();
    merge_sort(&mut vec, &mut aux);
    println!("{:?}", vec);
}

fn merge_sort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T], aux: &mut [T]) {
	let length = array.len();
	if length <= 1 {return;}

    // Insertion sort if length < 15
    if length <= 15 {
        insertion_sort(array)
    } else {
        merge_sort(&mut array[0..length/2], &mut aux[0..length/2]);
        merge_sort(&mut array[length/2..length], &mut aux[length/2..length]);
        
        // Only merge if out of order
        if array[(length/2) - 1] > array[length/2] {
            // Memory usage here
            // Create new vector, and free the space after we are out of this if block
            // Space-wise it's O(n) extra space for the new vector 
            // Unsure if about costs to create the new vector, and free memory afterwards
            // Can keep the same vector throughout

            aux.copy_from_slice(&array);
            merge(&array[0..length/2], &array[length/2..length], &mut aux[..]);
            array.copy_from_slice(&aux);
        }
    }
}

fn merge<T: Copy + PartialOrd + std::fmt::Debug>(left: &[T], right: &[T], array: &mut [T]) {
    assert_eq!(left.len() + right.len(), array.len());
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
            array[k] = right[j];
            j += 1;
        }
    }

    println!("{:?}", left);
    println!("{:?}", right);
    println!("{:?}", array);
    println!("------");
}

fn insertion_sort<T: Copy + PartialOrd + std::fmt::Debug>(vec: &mut [T]) {
    let N = vec.len();

    for i in 0..N {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j-1);
            j = j - 1;
        }
    }        
}