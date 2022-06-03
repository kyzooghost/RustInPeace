#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p306 - Exercises

// 2.3.23

// Take the median from up to 9 items, use as pivot
// Cutoff to insertion sort for small subarrays

use rand::{thread_rng};

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    
    quicksort(&mut vec);
    println!("{:?}", vec);
}

fn quicksort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    let length = array.len();

    // Insertion sort if <= 16 elements
    if length <= 16 {
        insertion_sort(array);
        return
    }

    // Find median of 9 numbers and swap into pivot position
    let mut rng = thread_rng();
    let random_indexes = rand::seq::index::sample(&mut rng, array.len(), 9).into_vec();
    let mut random_elements = Vec::new();
    for index in random_indexes.clone() {random_elements.push(array[index]);}
    let pivot = quickselect(&random_elements, 5);
    for i in 0..9 {if pivot == random_elements[i] {array.swap(0, random_indexes[i])}}

    let mut p = 1;
    let mut i = 1;
    let mut j = length - 1;
    let mut q = length - 1;

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

fn quickselect<T: Copy + Ord + std::fmt::Debug>(array: &[T], index: usize) -> T {
    let length = array.len();
    if length == 1 {return array[0]}

    let j = partition(&mut array.to_vec());

    if index == j {
        return array[j]
    } else if index < j {
        quickselect(&array[0..j], index)
    } else {
        quickselect(&array[j+1..length], index - j - 1)
    }
}

fn partition<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) -> usize {
    // Can only do median-of-three if there are >= 3 elements
    let pivot = array[0];
    let mut i = 1;
    let mut j = array.len() - 1;

    loop {
        // Scan --> until find array[i] > pivot
        while array[i] < pivot {
            if i == array.len() - 1 {break;}
            i += 1;
        }

        // Scan <-- until find array[j] < pivot>
        while pivot < array[j] {
            j -= 1;
        }

        if i >= j {break;}

        array.swap(i, j);
    }

    array.swap(0, j);

    j
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