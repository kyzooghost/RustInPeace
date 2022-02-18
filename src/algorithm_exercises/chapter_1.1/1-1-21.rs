// Write a program that reads in lines from standard input with each line containing
// a name and two integers and then uses printf() to print a table with a column of
// the names, the integers, and the result of dividing the first by the second, accurate to
// three decimal places. You could use a program like this to tabulate batting averages for
// baseball players or grades for students.

// Read in lines from standard input - each line contain name + 2 integers
// Print table with a column of names, integers, and result of dividing first by second, accurate to three decimal places

fn main() {
  let mut vec = Vec::new();
  
  for i in 0..5 {
    vec.push( input_data() );
  }

  println!(
    "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
    "NAME", "x", "y", "x/y"
  );  

  println!("");

  for (name, x, y) in vec {
    println!(
      "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
      name, x, y, x/y
    );  
  }
  
}

fn input_data() -> (String, i32, i32) {
  use std::io;
  println!("Type a name and two(2) integers");
  let mut input = String::new();
    
  io::stdin()
      .read_line(&mut input)
      .expect("Failed to read input");

  input = input.clone();

  let split_input: Vec<&str> = input.trim().splitn(3, ' ').collect();

  let name = split_input[0];

  let first_integer = split_input[1].parse::<i32>()
    .expect("Did not read first integer"); 

  let second_integer = split_input[2].parse::<i32>()
    .expect("Did not read second integer");

  (name.to_string(), first_integer, second_integer)
}

// https://stackoverflow.com/questions/30379341/how-to-print-well-formatted-tables-to-the-console