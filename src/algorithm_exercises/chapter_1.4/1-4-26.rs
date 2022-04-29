#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.26 3-collinearity. 

// Suppose that you have an algorithm that takes as input N distinct points in the plane 
// and can return the number of triples that fall on the same line. 
// Show that you can use this algorithm to solve the 3-sum problem. 

// Strong hint: Use algebra to show that (a, a3), (b, b3), and (c, c3) are collinear if and only if a + b + c = 0.

// So for 3-sum problem, if three numbers a + b + c = 0, it is a 3 sum
// How many groupings of a, b and c can you find that have this property?
// Let's take it for true that (a, a3), (b, b3), and (c, c3) are collinear 
// (we will need to prove this later, that this holds true if and only if a + b + c = 0)
// Then every number can be mapped as a distinct point (x, x3) on the plane
// The algorithm will return the number of collinear triples
// The collinear triples can be mapped 1:1 back to the numbers
// Hence # of collinear triples = Number of groupings of three numbers summing to 0 = Solution to three sum problem

// Hence we just prove the strong hint

// If (a, a3), (b, b3), and (c, c3) are collinear
// | ( a3 - b3 ) / (a - b) | = | ( c3 - b3 ) / (c - b) |
// | a2 + b2 + ab | = | c2 + b2 + bc |

// a + b + c = 0 => c = 0 - b - a
// | a2 + b2 + ab | = | a2 + b2 + 2ab + b2 - b(b + a) |
// | a2 + b2 + ab | = | a2 + b2 + ab |
// Q.E.D

use rand::{thread_rng, Rng};

fn main() {
    // Initialize data structures & variables
    let mut rng = thread_rng();

    let ARRAY_SIZE:i32 = 100;
    let MAX_NUMBER:i32 = 100;
    let mut vec: Vec<i32> = (0..ARRAY_SIZE).map(|_| rng.gen_range(0..MAX_NUMBER)).collect();
    vec.sort();

    // Start timer here
    use std::time::Instant;
    let now = Instant::now();

    // println!("{:?}", binarySearch(&vec, 50));

    let elapsed1 = now.elapsed();
    println!("Time for getLocalMinimum: {:.2?}", elapsed1);
}