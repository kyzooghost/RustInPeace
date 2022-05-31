#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p305 - Exercises

// 2.3.17

// Sentinels. Modify the code in Algorithm 2.5 
// to remove both bounds checks in the inner while loops. 
// The test against the left end of the subarray is redundant 
// since the partitioning item acts as a sentinel (v is never less than a[lo]). 
// To enable removal of the other test, put an item whose key is the largest 
// in the whole array into a[length-1] just after the shuffle. 
// This item will never move (except possibly to be swapped with an item having the same key) 
// and will serve as a sentinel in all subarrays involving the end of the array. 

// Note : When sorting interior subarrays, the leftmost entry in the subarray to the right 
// serves as a sentinel for the right end of the subarray.

fn main() {
    use rand::{thread_rng, seq::SliceRandom};
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    

    // Shuffle
    vec.shuffle(&mut thread_rng());

    // Place biggest value at array end
    let mut max = vec[0];
    let mut max_index = 0;

    for (i, value) in vec.iter().enumerate() {
        if value > &max {
            max = value;
            max_index = i;
        }
    }

    let length = vec.len();
    vec.swap(max_index, length - 1);

    quicksort(&mut vec);
    println!("{:?}", vec);
}

fn quicksort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {

    if array.len() <= 1 {
        return
    }

    let j = partition_sentinel(array);
    let length = array.len();
    quicksort(&mut array[0..j]);
    quicksort(&mut array[j+1..length]);
}

fn partition<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) -> usize {
    let mut i = 1;
    let mut j = array.len() - 1;
    let pivot = array[0];

    loop {
        // Scan --> until find array[i] > pivot
        while array[i] < pivot {
            if i == array.len() - 1 {break;}
            i += 1;
        }

        // Scan <-- until find array[j] < pivot>
        while pivot < array[j] {
            if j == 0 {break;}
            j -= 1;
        }

        if i >= j {break;}

        array.swap(i, j);
    }

    array.swap(0, j);

    j
}

// The test against the left end of the subarray is redundant 
// since the partitioning item acts as a sentinel (v is never less than a[lo]). 

// To enable removal of the other test, put an item whose key is the largest 
// in the whole array into a[length-1] just after the shuffle. 
// This item will never move (except possibly to be swapped with an item having the same key) 
// and will serve as a sentinel in all subarrays involving the end of the array. 

// Note : When sorting interior subarrays, the leftmost entry in the subarray to the right 
// serves as a sentinel for the right end of the subarray.

// Find maximum element -> O(N)
// Shuffle -> O(N) and in-place via Fisher-Yates shuffle
// Move maximum element to end of array
// This is your sentinel for the right side, for any subarray involving the end of the array
// Ok the right side will always be >= anything in the left subarray, if your maximum element is
// guaranteed to be at the end of the array

// So the right sentinel will never be part of your partitioned array, but you need
// access to it regardless. So I don't think I can do it feeding just a mutable array slice
// to partition

fn partition_sentinel<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) -> usize {
    let mut i = 1;
    let mut j = array.len() - 1;
    let pivot = array[0];

    loop {
        // Scan --> until find array[i] > pivot
        while array[i] < pivot {
            // You have to guarantee that the index does not go past the end of the array 
            // You need the whole array, not an array slice then
            // Because i can increment to one beyond end of the array
            // Then j will be whatever
            // loop will break on i >= j
            // array.swap(0, j) will still be in-bounds

            if i == array.len() - 1 {break;}
            i += 1;
        }

        // Scan <-- until find array[j] < pivot>
        // If j == 0, pivot == array[j], so will stop while loop, and i >= 0, so will break
        while pivot < array[j] {
            j -= 1;
        }

        if i >= j {break;}

        array.swap(i, j);
    }

    array.swap(0, j);

    j
}