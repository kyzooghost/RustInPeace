use std::{io, char};

fn main() {
  let num = input_number();
  println!("You typed {}", num);
  println!("In binary this is {}", integer_to_binary_string(num));
  println!("In binary this is {:b}", num);
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

fn integer_to_binary_string(integer: i32) -> String {
  // Successive division by two method
  let mut reversed_binary_string = String::new();
  if integer == 0 {return reversed_binary_string}
  else {
    let mut temp_integer = integer;
    while temp_integer > 0 {
      let binary_digit = char::from_digit(temp_integer as u32 % 2, 10);
      reversed_binary_string.push(binary_digit.unwrap()); 
      temp_integer /= 2;
    }
    reversed_binary_string.chars().rev().collect()
  }
}