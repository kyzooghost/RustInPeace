#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p303 - Exercises

// 2.3.9

// What happens when quicksort run on array of just two distinct keys, and what about three distinct keys?

// Two distinct keys, A and B, B > A
// [A B A B A B A B]
// If choose A as first partition, 
// [A A A] and [B B B B] become the partitions
// So then basically the same as just having one distinct key from this point on

// Three distict keys
// [A ..] [B C ..]

// Will quickly tend to subarrays with just one distinct element => Use quicksort with three-way
// partitioning to improve performance from O(N lg N) to O(N)

fn main() {
    use rand::{thread_rng, seq::SliceRandom};
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    vec.shuffle(&mut thread_rng());
    quicksort_3_way(&mut vec);
    println!("{:?}", vec);
}

fn quicksort_3_way<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {

    if array.len() <= 1 {
        return
    }

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