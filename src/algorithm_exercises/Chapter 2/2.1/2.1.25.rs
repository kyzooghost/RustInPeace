#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p249 - Selection sort
// p251 - Insertion sort
// p259 - Shell sort
// p265 - Exercises

// 2.1.25  

// Insertion sort without exchange

fn main() {
    let vec = vec![35, 39, 49, 75, 89, 19, 78, 85, 18, 84];
    println!("{:?}", "B" < "A");

    let vec = vec!["E", "A", "S", "Y", "S", "H", "E", "L", "L", "S", "O", "R", "T", "Q", "U", "E", "S", "T", "I", "O", "N"];
    let sorted_vec = insertionSortWithoutExchange(vec);
    println!("{:?}", sorted_vec);
}

fn insertionSortWithoutExchange(mut vec: Vec<&str>) -> Vec<&str> {
    let N = vec.len();

    // Put smallest element in first index
    let mut min_value = vec[0];
    let mut min_index = 0;

    for i in 0..N {
        if vec[i] < min_value {
            min_value = vec[i];
            min_index = i;
        }
    }

    vec.swap(0, min_index);

    // Insertion sort loop
    
    // Basically storing the element you are inspecting
    // Shift all the elemnts to the right until you get to the position you want to insert at
    // Insert the element there

    for i in 1..N {
        let aux = vec[i];
        let mut j = i;

        while vec[j] < vec[j - 1] {
            vec[j] = vec[j - 1];
            j = j - 1;
        }

        vec[j] = aux;
    }        

    vec
}

fn insertionSort(mut vec: Vec<&str>) -> Vec<&str> {
    let N = vec.len();

    for i in 0..N {
        let mut j = i;

        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j-1);
            j = j - 1;
        }
    }        

    vec
}