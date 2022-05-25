#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p284 - Exercises

// 2.2.2

// E A S Y Q U E S T I O N
// Top-down mergesort trace

// E A | S Y | Q U | E S | T I | O N
// A E | S Y | Q U | E S | I T | N O
// A E S Y | E Q S U | I N O T
// A E E Q S S U Y | I N O T
// A E E I N O Q S S T U Y


fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    let length = vec.len();
    merge_sort(&mut vec, 0, length);
    println!("{:?}", vec);
}

// fn merge_sort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
// 	let length = array.len();
// 	if length <= 1 {return;}

// 	merge_sort(&mut array[0..length/2]);
// 	merge_sort(&mut array[length/2..length]);
 
//     // We are creating new array with each merge_sort call, this is not really in-place
//     // But have to create a new array 'y', because Rust won't let us have more than one mutable reference
//     // Memory safety > memory efficiency here
// 	let mut y: Vec<T> = array.to_vec();
// 	merge(&array[0..length/2], &array[length/2..length], &mut y[..]);
// 	array.copy_from_slice(&y);

//     // I can't do the following because Rust won't let me use multiple mutable references to the original array
//     // So I need to use unsafe Rust to do this in a memory efficient manner, and I can't be fucked fucking around 
//     // with unsafe Rust just to implement merge sort. Double linked lists is enough for me.
// 	// merge(&array[0..length/2], &array[length/2..length], array);
// }

fn merge_sort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T], low: usize, high: usize) {
	if high <= low {return;}

    let length = array.len();
    let mid = (low + high) / 2;

	merge_sort(&mut array[0..length/2], low, mid);
	merge_sort(&mut array[length/2..length], mid + 1, high);
 
	let mut y: Vec<T> = array.to_vec();
	merge(&array[0..length/2], &array[length/2..length], &mut y[..]);
	array.copy_from_slice(&y);
}

// Ah yes, you do need a mutable ref in Rust, to get over 'moving'
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

    println!("{:?}", left);
    println!("{:?}", right);
    println!("{:?}", array);
    println!("------");
}