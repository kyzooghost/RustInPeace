#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p389 Exercises

// 3.1.1

// Write a client that creates a symbol table mapping letter grades to numerical scores, as in the table below, then reads from standard input a list of letter grades and computes and prints the GPA (the average of the numbers corresponding to the grades).

// Symbol table to map letter grade to numerical score
// Client that takes series of grades, and computes GPA

// I cbf using linked lists, lets used the ordered array

// p366 ST API
// p379 + 381 Ordered array ST implementation

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Grade {
    F,
    D,
    C_Minus,
    C,
    C_Plus,
    B_Minus,
    B,
    B_Plus,
    A_Minus,
    A,
    A_Plus,
}

pub struct BinarySearchST<T, U> {
    size: usize,
    keys: Vec<T>,
    values: Vec<U>
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug, 
    U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    BinarySearchST<T, U> {

    pub fn size(&self) -> usize {self.size}
    pub fn isEmpty(&self) -> bool {self.size == 0}
    pub fn new() -> Self {
        BinarySearchST {
            size: 0,
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    // How many keys are below (if key not found) or equal (if key found)
    pub fn rank(&self, key: T) -> usize {
        if self.isEmpty() {return 0}

        let mut low = 0;
        let mut high = self.size - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if key < self.keys[mid] {
                if mid == 0 {break} // usize can't be negative, so break out of edge-case
                high = mid - 1
            }
            else if key > self.keys[mid] {low = mid + 1}
            else {return mid}
        }

        return low;
    }

    pub fn get(&self, key: T) -> Option<U> {
        if self.isEmpty() {return None}
        let i = self.rank(key);
        if i < self.size && self.keys[i] == key {return Some(self.values[i])}
        else {return None}
    }

    pub fn put(&mut self, key: T, value: U) {
        // Find rank
        let i = self.rank(key);

        // If match current key, change the value
        if i < self.size && self.keys[i] == key {
            self.values[i] = value;
            return
        }

        // For all keys and values entries from index i onwards, shift one right
        let mut j = self.size;

        // Need to push to keys[] and values[] or else get out-of-index error
        // Ok to push to end, because this is getting re-assigned in the below while loop
        self.keys.push(key);
        self.values.push(value);

        while j > i {
            self.keys[j] = self.keys[j - 1];
            self.values[j] = self.values[j - 1];
            j -= 1;
        }

        // Insert at [i]

        self.keys[i] = key;
        self.values[i] = value;

        self.size += 1;
    }
  
}

fn main() {
    println!("{:?}", get_GPA(vec![
        Grade::A_Plus, 
        Grade::F, 
        Grade::D, 
        ]));
}

fn get_GPA(grades: Vec<Grade>) -> f32 {
    let mut GradeST = BinarySearchST::new();
    GradeST.put(Grade::A_Plus, 4.33);
    GradeST.put(Grade::A, 4.00);
    GradeST.put(Grade::A_Minus, 3.67);
    GradeST.put(Grade::B_Plus, 3.33);
    GradeST.put(Grade::B, 3.00);
    GradeST.put(Grade::B_Minus, 2.67);
    GradeST.put(Grade::C_Plus, 2.33);
    GradeST.put(Grade::C, 2.00);
    GradeST.put(Grade::C_Minus, 1.67);
    GradeST.put(Grade::D, 1.00);
    GradeST.put(Grade::F, 0.00);

    let mut total_grade = 0.0;
    let num_of_grades = grades.len();
    for grade in grades {total_grade += GradeST.get(grade).unwrap();}
    
    total_grade / num_of_grades as f32
}