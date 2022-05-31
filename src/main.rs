#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p305 - Exercises

// 2.3.16

// Best case - Each partition splits the array into two

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