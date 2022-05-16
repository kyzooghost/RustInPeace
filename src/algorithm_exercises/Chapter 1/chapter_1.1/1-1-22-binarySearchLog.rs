// Write a version of BinarySearch that uses the recursive rank() given on page
// 25 and traces the method calls. Each time the recursive method is called, print the argument
// values lo and hi, indented by the depth of the recursion. Hint: Add an argument
// to the recursive method that keeps track of the depth.

fn main() {
  let v = vec![1, 2, 3, 4, 5, 6, 7, 9, 11, 12, 16, 17, 18,  21, 24, 25, 26, 32, 34, 36, 37, 38, 39, 40, 45, 46, 47,  53, 56, 61, 63, 65, 66, 67, 68, 69, 70, 79, 81, 83, 85,  86, 87, 89, 93, 94, 95, 96, 97, 98];

  let len = v.len() as i32;
  let num = 81;

  if let Some(i32) = binary_search(&v, 0, len - 1, num, 0) {
    println!("Index of {} is {:?}", num, i32)
  } else {
    // println!("Index of {} is {:?}", num, binary_search(&v, 0, len - 1, num))
    println!("The number you provided could not be found")
  }

  // println!("Index of {} is {:?}", num, binary_search(&v, 0, len - 1, num));
}

fn binary_search(_array: &[i32], _start: i32, _end:i32, _x:i32, _depth: i32) -> Option<i32> {
  // Print argument
  // Create string

  let mut indent = String::from("");

  for _i in 0.._depth {
    indent.push_str(" ");
  }

  println!("{}Lo: {}Hi: {}",indent, _start, _end);

  if _start > _end {return None}

  let mid = (_start + _end) / 2;

  if _x < _array[mid as usize] {
    return binary_search(_array, _start, mid - 1, _x, _depth + 1)
  }

  else if _x > _array[mid as usize] {
    return binary_search(_array, mid + 1, _end, _x, _depth + 1)
  }

  else {return Some(mid)}

}