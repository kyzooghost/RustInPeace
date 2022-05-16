fn main() {
  let v = vec![1, 2, 3, 4, 5, 6, 7, 9, 11, 12, 16, 17, 18,  21, 24, 25, 26, 32, 34, 36, 37, 38, 39, 40, 45, 46, 47,  53, 56, 61, 63, 65, 66, 67, 68, 69, 70, 79, 81, 83, 85,  86, 87, 89, 93, 94, 95, 96, 97, 98];

  println!("Mean: {:?}", find_mean(&v));
  println!("Median: {:?}", find_median(&v));
  println!("Mode: {:?}", find_mode(&v));
}

fn find_mode(numbers: &[i32]) -> Vec<&i32> {
  use std::collections::HashMap;
  let mut number_to_count = HashMap::new();
  let mut mode = vec![];
  let mut max_count:i32 = 1; // Default 1

  // Create hash map with numbers : number_of_occurences
  for num in numbers {
    let count = number_to_count.entry(num).or_insert(0);
    *count += 1;
  }

  // Iterate over hash map, determine your max_count
  for (_key, value) in &number_to_count {
    if value > &max_count {max_count = *value;}
  }

  // Iterate over hash map, collect modes
  for (key, value) in &number_to_count {
    if value == &max_count {
      mode.push(key.clone());
    }
  }

  mode
}

fn find_median(numbers: &[i32]) -> f32 {
  let len = numbers.len();
  let mut median:f32;

  if len % 2 == 0 {
    median = (numbers[len/2] + numbers[len/2 + 1]) as f32;
    median = median * 0.5;
  } else {
    median = numbers[len/2 + 1] as f32;
  }

  median
}

fn find_mean(numbers: &[i32]) -> f32 {
  let mut sum: f32 = 0.0;
  let len = numbers.len() as f32;

  for number in numbers {
    sum += *number as f32;
  }

  sum / len
}