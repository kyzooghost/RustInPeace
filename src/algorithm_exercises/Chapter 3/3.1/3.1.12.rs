#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p389 Exercises

// 3.1.12

// Modify BinarySearchST to maintain one array of Item objects that contain keys and values,
// rather than two parallel arrays. Add a constructor that takes an array of Item values
// as arguments and uses mergesort to sort the array

// p363 ADT for ST
// p366 ST API
// p379 + 381 Ordered array ST implementation

#[derive(Clone, Copy, Debug)]
pub struct Item<T, U> {
    key: T,
    value: U
}

pub struct BinarySearchST<T, U> {
    size: usize,
    array: Vec<Item<T, U>>
}

impl<T: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug, 
    U: Copy + Clone + PartialOrd + PartialEq + std::fmt::Debug> 
    BinarySearchST<T, U> {

    pub fn size(&self) -> usize {self.size}
    pub fn isEmpty(&self) -> bool {self.size == 0}

    pub fn new(mut vec: Vec<Item<T, U>>) -> Self {
        let mut vec_to_be_sorted = vec.clone();
        
        let mut newST = BinarySearchST {
            size: vec.len(),
            array: vec
        };

        // Merge sort input vector by key
        newST.merge_sort(&mut vec_to_be_sorted);
        newST.array = vec_to_be_sorted;
        newST
    }

    fn merge_sort(&self, array: &mut [Item<T, U>]) {
        let mut count = 0;
        let length = array.len();
        if length <= 1 {return}
    
        self.merge_sort(&mut array[0..length/2]);
        self.merge_sort(&mut array[length/2..length]);
     
        let mut y: Vec<Item<T,U>> = array.to_vec();
        self.merge(&array[0..length/2], &array[length/2..length], &mut y[..]);
        array.copy_from_slice(&y);
    }
    
    fn merge(&self, left: &[Item<T, U>], right: &[Item<T, U>], array: &mut [Item<T, U>]) {
        assert_eq!(left.len() + right.len(), array.len());
        let mut i = 0;
        let mut j = 0;
        let length = array.len();
    
        for k in 0..length {
            if i >= left.len() {
                array[k] = right[j];
                j += 1;
            } else if j >= right.len() {
                array[k] = left[i];
                i += 1;
            } else if left[i].key < right[j].key {
                array[k] = left[i];
                i += 1;
            } else {
                // left[i] > right[j]
                array[k] = right[j];
                j += 1;
            }
        }
    
    }

    // How many keys are below (if key not found) or equal (if key found)
    pub fn rank(&mut self, key: T) -> usize {
        if self.isEmpty() {return 0}

        let mut low = 0;
        let mut high = self.size - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if key < self.array[mid].key {
                if mid == 0 {break} // usize can't be negative, so break out of edge-case
                high = mid - 1
            }
            else if key > self.array[mid].key {
                low = mid + 1
            }
            else {
                return mid
            }
        }

        return low;
    }

    pub fn get(&mut self, key: T) -> Option<U> {
        if self.isEmpty() {return None}
        let i = self.rank(key);
        if i < self.size && self.array[i].key == key {return Some(self.array[i].value)}
        else {return None}
    }

    // pub fn put(&mut self, key: T, value: U) {
    //     // Find rank
    //     let i = self.rank(key);

    //     // If match current key, change the value

    //     if i < self.size && self.keys[i] == key {
    //         self.values[i] = value;
    //         return
    //     }

    //     // For all keys and values entries from index i onwards, shift one right
    //     let mut j = self.size;

    //     // Need to push to keys[] and values[] or else get out-of-index error
    //     // Ok to push to end, because this is getting re-assigned in the below while loop
    //     self.keys.push(key);
    //     self.values.push(value);

    //     while j > i {
    //         self.keys[j] = self.keys[j - 1];
    //         self.values[j] = self.values[j - 1];
    //         j -= 1;
    //     }

    //     // Insert at [i]

    //     self.keys[i] = key;
    //     self.values[i] = value;

    //     self.size += 1;
    // }
  
}

fn main() {
    let vec = vec![
        Item{key: "E", value: 0},
        Item{key: "A", value: 1},
        Item{key: "S", value: 2},
        Item{key: "Y", value: 3},
        Item{key: "Q", value: 4},
        Item{key: "U", value: 5},
        Item{key: "T", value: 8},
        Item{key: "I", value: 9},
        Item{key: "O", value: 10},
        Item{key: "N", value: 11},
    ];

    let mut ST = BinarySearchST::new(vec);

    println!("{:?}", ST.array);
}