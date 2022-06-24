#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p391 Exercises

// 3.1.26

// Frequency count from a dictionary. Modify FrequencyCounter to take the name of a dictionary file as its argument, count frequencies of the words from standard input that are also in that file, and print two tables of the words with their frequencies, one sorted by frequency, the other sorted in the order found in the dictionary file.

// If you use BinarySearchST with key as word, that is sorted in dictionary file order
// If you use BinarySearchST with frequency as key, that is sorted in frequency

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