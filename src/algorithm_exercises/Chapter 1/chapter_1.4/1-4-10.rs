#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.10

// Modify binary search so that it always returns the element with the smallest index 
// that matches the search element (and still guarantees logarithmic running time).

fn main() {
    let vec = vec![1, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 5, 6, 7, 8, 9, 10, 10, 10];
    println!("{:?}", binary_search(&vec, 0, vec.len(), 3));
}

fn binary_search(_vec: & Vec<i32>, _start: usize, _end: usize, _x:i32) -> Option<usize> {
    // Base case
    if _start > _end {return None}
    
    // Constant time
    let mid = (_start + _end) / 2;
    if mid == 0 {return Some(mid)}

    // O(log N) because you keep halving the array you are searching
    // Make it go through the recursive call for the lower half, if it is not the smallest index
    if _x < _vec[mid as usize] || _x == _vec[mid - 1 as usize] {
      return binary_search(_vec, _start, mid - 1, _x)
    }
  
    else if _x > _vec[mid as usize] {
      return binary_search(_vec, mid + 1, _end, _x)
    }
  
    else {return Some(mid)}  
}