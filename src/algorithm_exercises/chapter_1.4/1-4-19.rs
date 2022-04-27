#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.19 Local minimum of a matrix

// Given an N-by-N array a[] of N 2 distinct integers, 
// design an algorithm that runs in time proportional to N to find a local minimum: 
// a pair of indices i and j such that a[i][j] < a[i+1][j], a[i][j] < a[i][j+1], a[i][j] < a[i-1][j], and a[i][j] < a[i][j-1]. 
// The running time of your program should be proportional to N in the worst case.

// Get a window - window is n/2 * n/2 area
// Find global max in the window  - ~O(2n)
// Four windows - ~O(8n)
// Find your biggest window

// Split that into four windows again
// Each n/4 * n/4 area
// Will take ~O(4n) here

// Keep halving window size, until get to single square

// ~Geometric sum that converges to ~O(16n)

use rand::{thread_rng, Rng};

// wo clone
// 2 - 9.5 micros
// 20 - 39 micros - 4x
// 200 - 240 micros - 6x
// 2000 - 3120 micros - ~12x
// 4000 - 9.44ms - 2.7x

// w clone
// 2 - 14.25 micros
// 20 - ~46 micros - 3.3x
// 200 - ~292 micros - 6.3x
// 2000 - 6110ms - 

// Still have a corner case that is not covered - the minimum it finds is not a local minimum (i.e. adjacent 0). 
// However there is a the same minimum which is a local minimum elsewhere on the matrix.
// It finds the minimum number, but if it happens to hone in on the matrix element with an adjacent clone, it will return None

// Suggested solution to this edge case
// If you have found multiple minimums in one layer, test if any of them are a local minimum, if yes, that is your solution
// Else recurse

// This is tedious af to implement, 2D and the corner & edge cases
// Acceptable compromise for O(n) vs O(n^2) performance?

fn main() {
    // Initialize data structures & variables
    let MATRIX_SIZE = 10;
    let MAX_NUMBER = 10;
    let mut rng = thread_rng();
    let matrix: Vec<Vec<u32>> = (0..MATRIX_SIZE)
    .map(|_| (0..MATRIX_SIZE)
            .map(|_| rng.gen_range(0..MAX_NUMBER))
            .collect())
    .collect();

    for row in matrix.clone() {
        println!("{:?}", row);
    }

    // Start timer here
    use std::time::Instant;
    let now = Instant::now();

    println!("Local minimum - {:?}", getLocalMinimum(matrix));
    let elapsed1 = now.elapsed();
    println!("Time for getLocalMinimum: {:.2?}", elapsed1);
}

fn getLocalMinimum(matrix: Vec<Vec<u32>>) -> Option<u32> {
    let mut minimum = matrix[0][0];

    let mut left = 0;
    let mut top = 0;
    let mut right = matrix.len() - 1;
    let mut bottom = matrix.len() - 1;
    let mut quadrant_size = (matrix.len() + 1) / 2;

    let findMinimumInQuadrant = |top: usize, left: usize, length: usize| -> u32 {
        let mut minimum = matrix[top][left];

        for i in top..top + length {
            if matrix[i][left] < minimum {minimum = matrix[i][left]}
        }

        for i in top..top + length {
            if matrix[i][left + length - 1] < minimum {minimum = matrix[i][left + length - 1]}
        }

        for i in left..left + length {
            if matrix[top][i] < minimum {minimum = matrix[top][i]}
        }

        for i in left..left + length {
            if matrix[top + length - 1][i] < minimum {minimum = matrix[top + length - 1][i]}
        }

        minimum
    };

    while bottom > top && right > left {

        let min1 = findMinimumInQuadrant(top, left, quadrant_size);
        let min2 = findMinimumInQuadrant(top, right - quadrant_size + 1, quadrant_size);
        let min3 = findMinimumInQuadrant(bottom - quadrant_size + 1, left, quadrant_size);
        let min4 = findMinimumInQuadrant(bottom - quadrant_size + 1, right - quadrant_size + 1, quadrant_size);


        if min1 < minimum {minimum = min1}
        if min2 < minimum {minimum = min2}
        if min3 < minimum {minimum = min3}
        if min4 < minimum {minimum = min4}

        match minimum {
            min if min == min1 => {
                bottom = top + quadrant_size - 1;
                right = left + quadrant_size - 1;
            },
            min if min == min2 => {
                left = right - quadrant_size + 1;
                bottom = top + quadrant_size - 1;
            },
            min if min == min3 => {
                top = bottom - quadrant_size + 1;
                right = left + quadrant_size - 1;
            },
            min if min == min4 => {
                left = right - quadrant_size + 1;
                top = bottom - quadrant_size + 1;
            },
            _=> {
                panic!("Should find minimum in a quadrant")
            }
        }

        quadrant_size = (bottom - top + 2) / 2;
    }

    // Should have single element at this point, left == right && top == bottom
    if bottom != top && right != left {panic!("should have reduced to one element")}

    println!("MINIMUM - {:?}", minimum);
    println!("TOP - {:?}", top);
    println!("LEFT - {:?}", left);
    println!("BOTTOM - {:?}", bottom);
    println!("RIGHT - {:?}", right);

    // Literally the corner cases - if last element is in the corner of the matrix
    if top == 0 && left == 0 {
        if matrix[top][left] < matrix[top+1][left] && matrix[top][left] < matrix[top][left+1] {return Some(minimum)} 
        else {return None}
    } else if top == 0 && right == matrix.len() - 1 {
        if matrix[top][right] < matrix[top+1][right] && matrix[top][right] < matrix[top][right-1] {return Some(minimum)} 
        else {return None}
    } else if bottom == matrix.len() - 1 && left == 0 {
        if matrix[bottom][left] < matrix[bottom-1][left] && matrix[top][left] < matrix[bottom][left+1] {return Some(minimum)} 
        else {return None}
    } else if bottom == matrix.len() - 1 && right == matrix.len() - 1 {
        if matrix[bottom][right] < matrix[bottom-1][right] && matrix[bottom][right] < matrix[bottom][right-1] {return Some(minimum)} 
        else {return None}
    }

    // Literally the edge case - if the last element is on an edge
    if top == 0 {
        if minimum < matrix[top][left-1] && minimum < matrix[top][left+1] && minimum < matrix[top+1][left] {return Some(minimum)}
        else {return None}
    } else if left == 0 {
        if minimum < matrix[top-1][left] && minimum < matrix[top+1][left] && minimum < matrix[top][left+1] {return Some(minimum)}
        else {return None}
    } else if bottom == matrix.len() - 1 {
        if minimum < matrix[bottom][left+1] && minimum < matrix[bottom][left-1] && minimum < matrix[bottom-1][left] {return Some(minimum)}
        else {return None}
    } else if right == matrix.len() - 1 {
        if minimum < matrix[top-1][right] && minimum < matrix[top+1][right] && minimum < matrix[top][right-1] {return Some(minimum)}
        else {return None}
    }

    // Cover case that element is not on corner or edge
    if minimum < matrix[top][left+1] && minimum < matrix[top][left-1] && minimum < matrix[top+1][left] && minimum < matrix[top-1][left] {
        return Some(minimum)
    } else {
        println!("{:?}", matrix[top][left+1]);
        println!("{:?}", matrix[top][left-1]);
        println!("{:?}", matrix[top+1][left]);
        println!("{:?}", matrix[top-1][left]);
        return None
    }
}