#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p285 - Exercises

// 2.2.12

// Sublinear extra space. 

// Develop a merge implementation that reduces 
// the extra space requirement to max(M, N/M), 
// based on the following idea: 

// Divide the array into N/M blocks of size M 
// (for simplicity in this description, assume 
// that N is a multiple of M). Then, (i) considering 
// the blocks as items with their 
// first key as the sort key, sort them using selection sort; 
// and (ii) run through the array merging the first block with the second, 
// then the second block with the third, and so forth.

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    sort(&mut vec, 10);
    println!("{:?}", vec);
}

fn sort<T: Copy + Ord + std::fmt::Debug>(mut array: &mut Vec<T>, N: usize) {
    // Divide array into M blocks of size N
    let length = array.len();
    assert!(N <= length, "N > array length");

    let mut left = array[0..N].to_vec();
    selection_sort(&mut left);
    array[0..N].copy_from_slice(&left);

    // Edge case
    if 2*N > length {
        let mut right = array[N..length].to_vec();
        selection_sort(&mut right);
        array[N..length].copy_from_slice(&right);
        merge(&left, &right, &mut array);
    } else {
        let mut i = N;

        while i < length {
            // Edge case - reached the edge literally
            if i + N > length {
                left = array[0..i].to_vec();
                let mut right = array[i..length].to_vec();
                selection_sort(&mut right);
                array[i..length].copy_from_slice(&right);
                merge(&left, &right, &mut array);
                return
            }

            left = array[0..i].to_vec();
            let mut right = array[i..i+N].to_vec();
            selection_sort(&mut right);
            array[i..i + N].copy_from_slice(&right);
            merge(&left, &right, &mut array[0..i+N]);

            i += N;
        }
    }
}

fn selection_sort<T: Copy + PartialOrd + std::fmt::Debug>(vec: &mut Vec<T>) {
    let N = vec.len();

    for i in 0..N {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j-1);
            j = j - 1;
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
}