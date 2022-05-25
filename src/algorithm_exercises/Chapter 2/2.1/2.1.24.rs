#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p249 - Selection sort
// p251 - Insertion sort
// p259 - Shell sort
// p265 - Exercises

// 2.1.24

// Insertion sort with sentinel

// Implementation of insertion sort that remove j>0 test in inner loop
// First put smallest item into position
// Avoid index-out-of-bounds test with sentinel


fn main() {
    let vec = vec![35, 39, 49, 75, 89, 19, 78, 85, 18, 84];
    println!("{:?}", "B" < "A");

    let vec = vec!["E", "A", "S", "Y", "S", "H", "E", "L", "L", "S", "O", "R", "T", "Q", "U", "E", "S", "T", "I", "O", "N"];
    let sorted_vec = insertionSortWithSentinel(vec);
    println!("{:?}", sorted_vec);
}

fn insertionSortWithSentinel(mut vec: Vec<&str>) -> Vec<&str> {
    let N = vec.len();

    // Put smallest element in first index
    let mut min_value = vec[0];
    let mut min_index = 0;

    for i in 0..N {
        if vec[i] < min_value {
            min_value = vec[i];
            min_index = i;
        }
    }

    vec.swap(0, min_index);

    // Classic insertion sort loop
    // Don't need the j > 0 check, because you know the smallest value is in vec[0]
    // Hence it is an invariant that vec[1] > vec[0]
    for i in 1..N {
        let mut j = i;

        while vec[j] < vec[j - 1] {
            vec.swap(j, j-1);
            j = j - 1;
        }
    }        

    vec
}

fn insertionSort(mut vec: Vec<&str>) -> Vec<&str> {
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