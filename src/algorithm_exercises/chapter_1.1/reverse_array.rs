fn main() {
  let v = vec![1, 2, 3, 4, 5, 6, 7, 9, 11, 12, 16, 17, 18,  21, 24, 25, 26, 32, 34, 36, 37, 38, 39, 40, 45, 46, 47,  53, 56, 61, 63, 65, 66, 67, 68, 69, 70, 79, 81, 83, 85,  86, 87, 89, 93, 94, 95, 96, 97, 98];

  println!("{:?}", reverse_array(&v))
}

fn reverse_array(numbers: &[i32]) -> Vec<i32> {
  let length = numbers.len();
  let mut new_numbers = numbers.to_vec();

  for i in 0..(length/2) {
    let temp = new_numbers[i];
    new_numbers[i] = new_numbers[length - 1 - i];
    new_numbers[length - 1 - i] = temp
  }

  new_numbers
}