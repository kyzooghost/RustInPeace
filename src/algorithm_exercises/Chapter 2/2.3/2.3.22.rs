#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p306 - Exercises

// 2.3.22

// Fast 3-way partitioning

// Keep items with equal keys at both left and right ends of subarray
// Add to inner partitioning loop - swap == elements to end of array, before usual comparison of a[i] and a[j]
// After partitioning done, add code to swap the item with equal keys into position

// Urgh it works, but this solution with the catch for length == 2 feels wrong

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    
    quicksort(&mut vec);
    println!("{:?}", vec);
}


// [lo..p-1] == pivot == [q+1..length]
// [p..i-1] < pivot
// [j+1..q] < pivot
fn quicksort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    let length = array.len();
    if length <= 1 {return}
    if length == 2 {
        if array[0] > array[1] {array.swap(0,1)}
        return
    }

    let mut p = 1;
    let mut i = 1;
    let mut j = length - 1;
    let mut q = length - 1;

    let pivot = array[0];

    while i < j {
        if pivot == array[i] {
            array.swap(i, p);
            i += 1;
            p += 1;
        } else if pivot == array[j] {
            array.swap(j, q);
            j -= 1;
            q -= 1; 
        } else if array[i] < pivot {
            i += 1;
        } else if array[j] > pivot {
            j -= 1
        }

        array.swap(i, j);
    }

    while p > 0 && array[0] == pivot {
        array.swap(j - 1, p - 1);
        j -= 1;
        p -= 1;
    }

    while q < length - 1 && array[length - 1] == pivot {
        array.swap(i, q + 1);
        i += 1;
        q += 1;
    }

    quicksort(&mut array[0..j]);
    quicksort(&mut array[i..length]);
}