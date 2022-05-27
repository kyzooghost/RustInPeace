#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p285 - Exercises

// 2.2.16

// Natural mergesort. 

// Write a version of bottom-up mergesort that takes advantage of order 
// in the array by proceeding as follows each time it needs to find two arrays to merge: 
// find a sorted subarray (by incrementing a pointer until finding an entry that is 
// smaller than its predecessor in the array), then find the next, then merge them. 
// Analyze the running time of this algorithm in terms of the array size and the number of
// maximal increasing sequences in the array.

// Urgh, it's a straightforward concept but coding it in Rust without the for-loop syntax I'm used to is X.X

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    merge_sort(&mut vec);
    println!("{:?}", vec);
}

fn merge_sort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    let length = array.len();

    let mut i = 0;
    let mut left: &[T];
    let mut right: &[T];
    let mut low = 0;
    let mut mid = 0;
    let mut high = 0;

    loop {
        // Get left half
        while i < length - 1 && array[i] <= array[i + 1] {i += 1};
        i += 1;
        mid = i;

        if mid == length {
            if low == 0 {break} // 'subarray' is [0..length] => sorted
            else {
                low = 0;
                i = 0;
                continue; // subarray is [n..length] => start from left again
            }
        }

        left = &array[low..mid];

        // Get right half
        while i < length - 1 && array[i] <= array[i + 1] {i += 1};
        i += 1;
        high = i;
        right = &array[mid..high];

        // Merge
        let mut temp: Vec<T> = array.to_vec();
        merge(left, right, &mut temp[low..high]);
        array[low..high].copy_from_slice(&temp[low..high]);

        // Subarray is [n..length] => start from left again
        if high == length {
            i = 0;
            low = 0;
        } else {low = high + 1}
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