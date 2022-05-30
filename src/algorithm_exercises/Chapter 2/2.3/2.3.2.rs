#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p303 - Exercises

// 2.3.2

// Quicksort trace for E A S Y Q U E S T I O N

// E A S Y Q U E S T I O N
// E A S Y Q U E S T I O N
// E A E| Y Q U |S S T I O N
// A E |E| Y Q U S S T I O N

// A E E| |Y| Q U S S T I O N
// A E E| N Q U S S T I O Y

// A E E| |N| Q U S S T I O Y
// A E E| |N| I U S S T Q O Y
// A E E| I N U S S T Q O Y

// A E E I N| |U| S S T Q O Y
// A E E I N| |U| S S T Q O Y
// A E E I N| S S T Q O |U Y

// A E E I N| |S| S T Q O |U Y
// A E E I N| |S| O T Q S |U Y
// A E E I N| |S| O S Q T |U Y
// A E E I N| O S Q |S| T |U Y

// A E E I N| O S Q |S T U Y

// A E E I N O| S Q |S T U Y
// A E E I N O Q S S T U Y


fn main() {
    let vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N", "E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    println!("{:?}", merge_sort(&vec));
    // println!("{:?}", vec);
}