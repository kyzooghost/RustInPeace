#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p391 Exercises

// 3.1.28 - 30

// Ordered insertions. 
// Modify BinarySearchST so that inserting a key that is larger 
// than all keys in the table takes constant time 
// (so that building a table by calling put() for keys that are in order takes linear time).

// Do deleteMin() and deleteMax() operations
// Add assertions to BinarySearchST

pub struct Cache<T> {
    rank: usize,
    key: T
}

pub struct BinarySearchST<T, U> {
    size: usize,
    keys: Vec<T>,
    values: Vec<U>,
    cache: Option<Cache<T>>
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
            cache: None
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

    pub fn get(&mut self, key: T) -> Option<U> {
        // Check cache
        match &self.cache {
            Some(cache_struct) => {
                // Cache hit
                if cache_struct.key == key {
                    let rank = cache_struct.rank;
                    return Some(self.values[rank])
                }
            },
            None => {}
        }

        if self.isEmpty() {return None}
        let i = self.rank(key);
        if i < self.size && self.keys[i] == key {
            // Update cache
            self.cache = Some(Cache {
                key: key,
                rank: i
            });

            return Some(self.values[i])
        }
        else {return None}
    }

    pub fn put(&mut self, key: T, value: U) {
        // Check cache
        match &self.cache {
            Some(cache_struct) => {
                // Cache hit
                if cache_struct.key == key {
                    let rank = cache_struct.rank;
                    self.values[rank] = value;
                    return
                }
            },
            None => {}
        }

        // O(1) to insert key that is larger than current max
        if !self.isEmpty() && key > self.max().unwrap() {
            self.keys.push(key);
            self.values.push(value);
            self.size += 1;
            return
        }

        // Find rank
        let i = self.rank(key);

        // If match current key, change the value
        if i < self.size && self.keys[i] == key {
            self.values[i] = value;

            // Search hit -> Update cache
            self.cache = Some(Cache {
                key: key,
                rank: i
            });

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
        assert!(self.check_keys_sorted(), "keys not sorted");
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

        // Check cache
        match &self.cache {
            // Delete cache if cache hit
            Some(cache_struct) => {
                if cache_struct.key == key {
                    self.cache = None;
                }
            },
            None => {}
        }

        self.size -= 1;
        assert!(self.check_keys_sorted(), "keys not sorted");
    }

    pub fn deleteMin(&mut self) {
        if self.size == 0 {panic!("size 0, cannot delete")}

        let min = self.min().unwrap();

        // Check cache
        match &self.cache {
            // Delete cache if cache hit
            Some(cache_struct) => {
                if cache_struct.key == min {
                    self.cache = None;
                }
            },
            None => {}
        }

        self.keys.remove(0);
        self.values.remove(0);

        self.size -= 1;
        assert!(self.check_keys_sorted(), "keys not sorted");
    }

    pub fn deleteMax(&mut self) {
        if self.size == 0 {panic!("size 0, cannot delete")}

        let max = self.max().unwrap();

        // Check cache
        match &self.cache {
            // Delete cache if cache hit
            Some(cache_struct) => {
                if cache_struct.key == max {
                    self.cache = None;
                }
            },
            None => {}
        }

        self.keys.pop();
        self.values.pop();
        self.size -= 1;

        assert!(self.check_keys_sorted(), "keys not sorted");
    }

    fn check_keys_sorted(&self) -> bool {
        let mut i = 0;

        while i + 1 <= self.keys.len() - 1 {
            if self.keys[i + 1] < self.keys[i] {return false}
            i += 1;
        }

        true
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

    println!("{:?}", ST.size);
    println!("{:?}", ST.keys);
    println!("{:?}", ST.values);

    ST.deleteMax();
    ST.deleteMax();
    ST.deleteMin();

    println!("{:?}", ST.size);
    println!("{:?}", ST.keys);
    println!("{:?}", ST.values);
}