// Write a static method lg() that takes an int value N as argument and returns
// the largest int not larger than the base-2 logarithm of N. Do not use Math.

use std::io;

fn main() {
  let num = input_number();
  println!("You typed {}", num);
  println!("Log2 num < {}", lg(num));
}

fn input_number() -> i32 {
    println!("Type a number");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    match number.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number"),
    }
}

fn lg(integer: i32) -> i32 {
  if integer == 0 {panic!("Cannot log 0");}
  else {
    let binary_string = format!("{:b}", integer);
    let binary_string_length = binary_string.len();
    let mut bit_vector = Vec::new();

    for bit in binary_string.chars() {
      bit_vector.push(bit.to_digit(10).unwrap());
    }

    return binary_string_length as i32 - 1
  }
} 