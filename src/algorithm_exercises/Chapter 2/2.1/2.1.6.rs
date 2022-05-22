#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p249 - Selection sort
// p251 - Insertion sort
// p259 - Shell sort
// p264 - Exercises

// 2.1.6

// Which method runs faster for an array with all keys identical, selection sort or insertion sort?

// Selection sort is set performance - N^2 / 2 compares, N - 1 exchanges
// Insertion sort - No exchanges, N compares

// Insertion sort

fn main() {
    let vec = vec![35, 39, 49, 75, 89, 19, 78, 85, 18, 84];
    let sorted_vec = selectionSort(vec);
    println!("{:?}", sorted_vec);
}

fn selectionSort(mut vec: Vec<usize>) -> Vec<usize> {
    let N = vec.len();

    for i in 0..N {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j-1);
            j = j - 1;
        }
    }

    vec
}