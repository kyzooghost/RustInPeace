#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p303 - Exercises

// 2.3.1 

// Show, in the style of the trace given with partition(), 
// how that method partitions the array E A S Y Q U E S T I O N.

// E A S Y Q U E S T I O N
// E A S Y Q U E S T I O N
// E A E| Y Q U |S S T I O N
// A E |E| Y Q U S S T I O N

fn main() {
    let vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    println!("{:?}", merge_sort(&vec));
    // println!("{:?}", vec);
}