#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.11

// Add an instance method howMany() to StaticSETofInts(page 99) 
// that finds the number of occurrences of a given key in time
// proportional to log N in the worst case

// Ok the issue yesterday was that I wasn't measuring the time points properly
// Probably initializing the massive array is an O(n) process, and it distorts your measurements of the howMany() function runtime

fn main() {
    // Initialize
    use std::time::Instant;
    let a = StaticSetOfInts::new((0..1000000000).collect());
    let now = Instant::now();

    // Linear
    println!("{:?}", a.howMany_linear(&3));
    let elapsed1 = now.elapsed();
    println!("Linear: {:.2?}", elapsed1);

    // Log
    println!("{:?}", a.howMany_log(&3));
    let elapsed2 = now.elapsed();
    println!("Log: {:.2?}", elapsed2 - elapsed1);
}

struct StaticSetOfInts<i32> {
    vec: Vec<i32>
}

impl StaticSetOfInts<i32> {
    pub fn new(mut keys: Vec<i32>) -> Self {
        keys.sort();
        StaticSetOfInts{
            vec: keys
        }
    }

    pub fn contains(&self, key: &i32) -> bool {
        match self.rank(key) {
            Some(_) => true,
            None => false
        }
    }

    // ~2 log N
    pub fn howMany_log(&self, key: &i32) -> usize {
        // O(log N)
        let lowest_index = binary_search_lowest(&self.vec, 0, self.vec.len() - 1, key);

        match lowest_index {
            Some(index) => {
                //O(log N)
                let highest_index = binary_search_highest(&self.vec, 0, self.vec.len() - 1, key);
                return highest_index.unwrap() - lowest_index.unwrap() + 1
            },
            None => return 0
        }
    }    

    pub fn howMany_linear(&self, key: &i32) -> usize {
        let mut count = 0;

        for ele in &self.vec {
            if ele == key {count = count + 1}
        }

        count
    }

    fn rank(&self, key: &i32) -> Option<usize> {
        let mut lo = 0;
        let mut hi = self.vec.len() - 1;

        // This loop keeps halving the search field, O(log N)
        while lo <= hi {
            let mid = (hi + lo) / 2;
            if key < &self.vec[mid] {hi = mid - 1}
            else if key > &self.vec[mid] {lo = mid + 1}
            else {return Some(mid)}
        }

        None
    }
}

fn binary_search_lowest(_vec: & Vec<i32>, _start: usize, _end: usize, _x: &i32) -> Option<usize> {
    // Base case
    if _start > _end {return None}
    
    // Constant time
    let mid = (_start + _end) / 2;
    if mid == 0 {return Some(mid)}

    // O(log N) because you keep halving the array you are searching
    // Make it go through the recursive call for the lower half, if it is not the smallest index
    if _x < &_vec[mid as usize] || _x == &_vec[mid - 1 as usize] {
      return binary_search_lowest(_vec, _start, mid - 1, _x)
    }
  
    else if _x > &_vec[mid as usize] {
      return binary_search_lowest(_vec, mid + 1, _end, _x)
    }
  
    else {return Some(mid)}  
}

fn binary_search_highest(_vec: & Vec<i32>, _start: usize, _end: usize, _x: &i32) -> Option<usize> {
    // Base case
    if _start > _end {return None}
    
    // Constant time
    let mid = (_start + _end) / 2;
    if mid == _vec.len() - 1 {return Some(mid)}

    // O(log N) because you keep halving the array you are searching
    if _x < &_vec[mid as usize] {
      return binary_search_highest(_vec, _start, mid - 1, _x)
    }

    // Make it go through the recursive call for the upper half, if it is not the largest index
    else if _x > &_vec[mid as usize] || _x == &_vec[mid + 1 as usize] {
      return binary_search_highest(_vec, mid + 1, _end, _x)
    }
  
    else {return Some(mid)}  
}