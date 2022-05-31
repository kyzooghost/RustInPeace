#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p304 - Exercises

// 2.3.15

// Nuts and bolts. (G. J. E. Rawlins) 

// You have a mixed pile of N nuts and N bolts 
// and need to quickly find the corresponding pairs of nuts and bolts. 
// Each nut matches exactly one bolt, and each bolt matches exactly one nut. 
// By fitting a nut and bolt together, you can see which is bigger, 
// but it is not possible to directly compare two nuts or two bolts. 
// Give an efficient method for solving the problem.

// We're in a quicksort chapter, we can solve this with quicksort
// Arbitrarily choose either the nut or bolt to be the 'anchor' partition
// Using the 'anchor' partition, sort the other element into smaller, bigger and corresponding
// Then do the same using the other element

// So take any random nut
// Compare against all bolts to partition them into smaller, bigger and corresponding
// Use the corresponding bolt to partition the nuts into smaller and bigger
// Take your corresponding bolt-nut pair out, and repeat amongst your two partitions

// Shuffle nuts/bolts to avoid O(N^2) time cost

fn main() {
    use rand::{thread_rng, seq::SliceRandom};
    let mut vec = vec!["B", "A", "B" ,"A", "B", "A", "B", "A", "C", "A", "D", "A", "B", "R", "A"];
    // vec.shuffle(&mut thread_rng());
    quicksort_3_way(&mut vec);
    println!("{:?}", vec);
}

fn quicksort_3_way<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {

    if array.len() <= 1 {
        return
    }

    println!("{:?}", array);

    let mut lt = 0;
    let mut i = lt + 1;
    let mut gt = array.len() - 1;

    let partition = array[lt];

    while i <= gt {
        if array[i] < partition {
            array.swap(lt, i);
            lt += 1;
            i += 1;
        } else if array[i] > partition {
            array.swap(i, gt);
            gt -= 1;
        } else {
            i += 1;
        }
    }

    let length = array.len();
    quicksort_3_way(&mut array[0..lt]);
    quicksort_3_way(&mut array[gt + 1..length]);
}