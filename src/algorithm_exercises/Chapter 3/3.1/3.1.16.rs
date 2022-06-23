#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p389 Exercises

// 3.1.16 Implement delete() and floor() for BinarySearchST

pub struct BinarySearchST<T, U> {
    size: usize,
    keys: Vec<T>,
    values: Vec<U>,
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
            else if key > self.keys[mid] {
                low = mid + 1
            }
            else {
                return mid
            }
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

    pub fn min(&self) -> Option<T> {
        if self.isEmpty() {return None}
        Some(self.keys[0])
    }

    pub fn max(&self) -> Option<T> {
        if self.isEmpty() {return None}
        Some(self.keys[self.size - 1])
    }

    pub fn select(&self, index: usize) -> Option<T> {
        if self.isEmpty() || index >= self.size {return None}
        Some(self.keys[index])
    }

    // smallest key greater than or equal to key
    // rank => # of keys less than key
    pub fn ceiling(&self, key: T) -> T {
        let rank = self.rank(key);
        self.keys[rank]
    }

    // largest key less than or equal to key
    pub fn floor(&self, key: T) -> T {
        let rank = self.rank(key);
        if self.keys[rank] == key || rank == 0 {return self.keys[rank]}
        else {return self.keys[rank - 1]}
    }

    pub fn delete(&mut self, key: T) {
        let rank = self.rank(key);
        if self.keys[rank] != key {panic!("Key not found")}
        if self.size == 0 {panic!("size 0, cannot delete")}
        self.keys.remove(rank);
        self.values.remove(rank);
        self.size -= 1;
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
    // println!("{:?}", ST.delete("A"));
    println!("{:?}", ST.size);
    println!("{:?}", ST.keys);
    println!("{:?}", ST.values);
}