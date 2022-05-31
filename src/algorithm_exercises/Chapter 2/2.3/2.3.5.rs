#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p303 - Exercises

// 2.3.5

// Code to sort array that is known to consist of items having just two distinct keys
// Write quicksort with three-way partition

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    quicksort_3_way(&mut vec);
    println!("{:?}", vec);
}

// [0..lt] < partition
// [lt..gt] == partition
// [gt..length]
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