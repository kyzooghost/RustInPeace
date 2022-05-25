#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p249 - Selection sort
// p251 - Insertion sort
// p259 - Shell sort
// p265 - Exercises

// 2.1.16

// Certification. 
// Write a check() method that calls sort() for a given array and 
// returns true if sort() puts the array in order and leaves 
// the same set of objects in the array as were there initially, false otherwise. 

// Do not assume that sort() is restricted to move data only with exch(). 
// You may use Arrays.sort() and assume that it is correct.

// Lol just treat every array element as a merkle leaf {index: ..., value: ...}, then hash a concatenation of the merkle leaves
// Compare has of pre-sorted to post-sorted array

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