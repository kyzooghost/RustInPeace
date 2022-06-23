#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p389 Exercises

// 3.1.11

// Give a trace of the process of inserting the keys E A S Y Q U E S T I O N 
// into an initially empty table using BinarySearchST. How many compares are involved?

// E0 - 

// p363 ADT for ST
// p366 ST API
// p379 + 381 Ordered array ST implementation

pub struct BinarySearchST<T, U> {
    size: usize,
    keys: Vec<T>,
    values: Vec<U>,
    compares: usize
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
            compares: 0
        }
    }

    // How many keys are below (if key not found) or equal (if key found)
    pub fn rank(&mut self, key: T) -> usize {
        if self.isEmpty() {return 0}

        let mut low = 0;
        let mut high = self.size - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if key < self.keys[mid] {
                self.compares += 1; // Count compares
                if mid == 0 {break} // usize can't be negative, so break out of edge-case
                high = mid - 1
            }
            else if key > self.keys[mid] {
                self.compares += 1; // Count compares
                low = mid + 1
            }
            else {
                self.compares += 1; // Count compares
                return mid
            }
        }

        return low;
    }

    pub fn get(&mut self, key: T) -> Option<U> {
        if self.isEmpty() {return None}
        let i = self.rank(key);
        if i < self.size && self.keys[i] == key {return Some(self.values[i])}
        else {return None}
    }

    pub fn put(&mut self, key: T, value: U) {
        // Find rank
        let i = self.rank(key);

        // If match current key, change the value

        if i < self.size {self.compares += 1;} // Count compares

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
    let mut ST = BinarySearchST::new();
    ST.put("E", 0);
    ST.put("A", 1);
    ST.put("S", 2);
    ST.put("Y", 3);
    ST.put("Q", 4);
    ST.put("U", 5);
    ST.put("E", 6);
    ST.put("S", 7);
    ST.put("T", 8);
    ST.put("I", 9);
    ST.put("O", 10);
    ST.put("N", 11);
    println!("{:?}", ST.keys);
    println!("{:?}", ST.values);
    println!("{:?}", ST.compares);
}