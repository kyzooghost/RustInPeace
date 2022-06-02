#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p289 - Quick sort
// p291 - Partition
// p299 - Three-way quicksort
// p306 - Exercises

// 2.3.22

// Fast 3-way partitioning

// Keep items with equal keys at both left and right ends of subarray
// Add to inner partitioning loop - swap == elements to end of array, before usual comparison of a[i] and a[j]
// After partitioning done, add code to swap the item with equal keys into position

fn main() {
    let mut vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];    
    quicksort(&mut vec);
    println!("{:?}", vec);
}

fn quicksort<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) {
    if array.len() <= 1 {return}
    let j = partition(array);
    let length = array.len();
    quicksort(&mut array[0..j]);
    quicksort(&mut array[j+1..length]);
}

fn partition<T: Copy + Ord + std::fmt::Debug>(array: &mut [T]) -> usize {
    let pivot = array[0];
    let length = array.len() - 1;
    let mut p = 1;
    let mut i = 1;
    let mut q = length - 1;
    let mut j = length - 1;

    loop {
        while array[i] == pivot {
            array.swap(i, p);
            p += 1;
            i += 1;
        }

        while array[j] == pivot {
            array.swap(j, q);
            q -= 1;
            j -= 1;
        }

        // Scan --> until find array[i] >= pivot
        while array[i] < pivot {
            if i == length - 1 {break;}
            i += 1;
        }

        // Scan <-- until find array[j] <= pivot>
        while pivot < array[j] {
            j -= 1;
        }

        if i >= j {break;}

        array.swap(i, j);
    }

    while p > 0 && array[0] == pivot {
        array.swap(j, p - 1);
        j -= 1;
        p -= 1;
    }

    while q < length && array[length - 1] == pivot{
        array.swap(length - 1, i + 1);
        i += 1;
        q += 1;
    }

    j
}

