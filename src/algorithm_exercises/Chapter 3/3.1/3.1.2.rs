#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p389 Exercises

// 3.1.2

// Develop a symbol-table implementation ArrayST that uses 
// an (unordered) array as the underlying data structure to implement our basic symbol-table API.

// p363 ADT for ST
// p366 ST API
// p379 + 381 Ordered array ST implementation

// If we are using an unordered array, we can no longer use binary search
// So we're back to O(N) search and insertion

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

pub struct ArrayST<T, U> {
    size: usize,
    keys: Vec<T>,
    values: Vec<U>
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug, 
    U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    ArrayST<T, U> {

    pub fn size(&self) -> usize {self.size}

    pub fn isEmpty(&self) -> bool {self.size == 0}

    pub fn new() -> Self {
        ArrayST {
            size: 0,
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn get(&self, key: &T) -> Option<U> {
        if self.isEmpty() {return None}

        // Attempt linear search for key
        for (i, current_key) in self.keys.iter().enumerate() {
            // If search hit, return value
            if key == current_key {return Some(self.values[i])}
        }

        None
    }

    pub fn put(&mut self, key: T, value: U) {
        // Attempt linear search for key
        for (i, current_key) in self.keys.iter().enumerate() {
            // If search hit, change value and return
            if &key == current_key {
                self.values[i] = value;
                return
            }
        }

        // If linear search miss, push to arrays
        self.keys.push(key);
        self.values.push(value);
        self.size += 1;
    }

    pub fn contains(&self, key: &T) -> bool {
        match self.get(key) {
            Some(_) => true,
            None => false
        }
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
    let mut GradeST = ArrayST::new();
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
    for grade in grades {total_grade += GradeST.get(&grade).unwrap();}
    
    total_grade / num_of_grades as f32
}