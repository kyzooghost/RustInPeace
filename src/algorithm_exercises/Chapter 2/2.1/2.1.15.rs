#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p249 - Selection sort
// p251 - Insertion sort
// p259 - Shell sort
// p265 - Exercises

// 2.1.15

// Expensive exchange. 

// A clerk at a shipping company is charged with the task of 
// rearranging a number of large crates in order of the time 
// they are to be shipped out. Thus, the cost of compares is very low (just look at the labels) 
// relative to the cost of exchanges (move the crates). 

// The warehouse is nearly full—there is extra space sufficient 
// to hold any one of the crates, but not two. 

// What sorting method should the clerk use?

// Selection sort because guaranteed O(n) exchanges

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