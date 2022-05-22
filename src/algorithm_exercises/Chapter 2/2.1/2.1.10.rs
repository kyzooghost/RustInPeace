#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p249 - Selection sort
// p251 - Insertion sort
// p259 - Shell sort
// p264 - Exercises

// 2.1.10

// Why not use selection sort for h-sorting in shellsort?

// Because shell sort effectively runs through the entire array for each value of 'h'
// You are forced to do an exchange in each run

// Also as h decreases, array is partially sorted
// Insertion sort is faster than selection sort on partially sorted arrays

fn main() {
    let vec = vec![35, 39, 49, 75, 89, 19, 78, 85, 18, 84];
    println!("{:?}", "B" < "A");

    let vec = vec!["E", "A", "S", "Y", "S", "H", "E", "L", "L", "S", "O", "R", "T", "Q", "U", "E", "S", "T", "I", "O", "N"];
    let sorted_vec = shellSort(vec);
    println!("{:?}", sorted_vec);
}

fn shellSort(mut vec: Vec<&str>) -> Vec<&str> {
    let N = vec.len();
    let mut swapped = false;
    let mut h = 1;
    while h < N / 3 {h = 3*h + 1}

    while h >= 1 {
        for i in 0..N {
            let mut j = i;
            swapped = false;
    
            while j >= h && vec[j] < vec[j - h] {
                vec.swap(j, j-h);
                j = j - h;
                swapped = true;
            }

            if swapped {
                println!("i: {:?}, j: {:?}, h: {:?}", i, j, h);
                println!("{:?}", vec);
            }
        }
        
        h = h / 3;
    }

    vec
}