#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p284 - Exercises

// 2.2.1

// A E Q S U Y E I N O S T
// Merge() trace

// A E Q S U Y E I N O S T
// A E Q S U Y | E I N O S T

// A
// E Q S U Y | E I N O S T

// A E E
// Q S U Y | I N O S T

// A E E I N O
// Q S U Y | S T

// A E E I N O Q S S
// U Y | T

// A E E I N O Q S S T U Y
// 

fn main() {
    let mut vec = vec!["A", "E", "Q" ,"S", "U", "Y", "E", "I", "N", "O", "S", "T"];

    let length = vec.len();

    println!("{:?}", length);
    println!("{:?}", length / 2);
    let sorted_vec = merge(vec, 0, (length) / 2, length - 1);
    println!("{:?}", sorted_vec);
}

// Copy vec[low..high] to aux[low..high]
// "Abstractly" delete existing vec[low..high] values
// Split aux[low..high] into two halves
// From the left (assuming pre-sorted) of each half, add to vec[low..high]

// Can only merge two sorted halves
fn merge(mut vec: Vec<&str>, low: usize, mid: usize, high: usize) -> Vec<&str> {
    let mut i = low;
    let mut j = mid;

    // Copy vec[low..high] to aux[low..high]
    let mut aux: Vec<&str> = Vec::new();

    for _ in 0..low {
        aux.push("");
    }

    for k in low..high+1 {
        aux.push(vec[k])
    }

    for _ in high+1..vec.len() {
        aux.push("")
    }

    for k in low..high+1 {
        // Left half exhausted
        if i > mid {
            vec[k] = aux[j];
            j = j + 1;
        // Right half exhausted
        } else if j > high {
            vec[k] = aux[i];
            i = i + 1;
        // Right half > left half
        } else if aux[j] > aux[i] {
            println!("{:?} > {:?}", aux[j], aux[i]);
            vec[k] = aux[i];
            i = i + 1;
        // Left half > right half
        } else {
            println!("{:?} > {:?}", aux[i], aux[j]);
            vec[k] = aux[j];
            j = j + 1;
        }
    }

    vec
}
