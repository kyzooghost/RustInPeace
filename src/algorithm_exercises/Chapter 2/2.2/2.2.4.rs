#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p271 - In-place merge
// p273 - Top-down, mergesort
// p278 - Bottoms-up, mergesort
// p284 - Exercises

// 2.2.4

// Does the abstract in-place merge produce proper output if and only if the two
// input subarrays are in sorted order? Prove your answer, or provide a counterexample.

// Yes must be sorted

// [5, 3, 4, 1]
// [10, 3, 8, 4]

// [5]
// 3 4 1 | 10 3 8 4

// [5, 3, 4, 1, 10, 3, 8, 4]

fn main() {
}