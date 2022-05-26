#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p285 - Exercises

// 2.2.10

// Faster merge. 
// Implement a version of merge() that copies the 
// second half of a[] to aux[] in decreasing order 
// and then does the merge back to a[]. 

// This change allows you to remove the code to test that 
// each of the halves has been exhausted from the inner loop. 

// Note: The resulting sort is not stable (see page 341).

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    merge_sort(&mut vec);
    println!("{:?}", vec);
}

fn merge<T: Copy + PartialOrd + std::fmt::Debug>(left: &[T], right: &[T], array: &mut [T]) {
    assert_eq!(left.len() + right.len(), array.len());

    // Need to concat left and right into the same array
    let mut auxIndex = 0;
    let mut aux = array.to_vec();

    for i in 0..left.len() {
        aux[auxIndex] = left[i];
        auxIndex += 1;
    }

    for j in 0..right.len() {
        aux[auxIndex] = right[right.len() - 1 - j];
        auxIndex += 1;
    }

    let mut i = 0;
    let mut j = aux.len() - 1;
    let mut k = 0;
    
    // If left and right are sorted, j will never decrement beyond left.len() - 1
    // If j reaches left.len() - 1 first, it will stay stuck there. The first 'if' will always 
    // be true because the left half is sorted

    // Otherwise if the left half is all < the right half, i will be 'stuck' on
    // left.len() - 1 until j reaches left.len() - 1
    while i < left.len() {
        if aux[i] <= aux[j] {
            array[k] = aux[i];
            i += 1;
        } else {
            array[k] = aux[j];
            j -= 1;
        }
        k += 1;
    }
}

fn merge_sort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    let length = array.len();

    let mut size = 1;
    while size < length {
        let mut low = 0;
        while low < length - size {
            let mut y: Vec<T> = array.to_vec();
            merge(&array[low..low + size], &array[low + size..std::cmp::min(low + size + size, length)], &mut y[low..std::cmp::min(low + size + size, length)]);
            array.copy_from_slice(&y);
            low = low + size + size;
        }
        size += size;
    }
}