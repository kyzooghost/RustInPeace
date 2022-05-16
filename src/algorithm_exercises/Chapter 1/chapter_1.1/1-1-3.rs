// Write a program that takes three integer command-line arguments and prints equal if all three are equal, and not equal otherwise

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {panic!("You did not get 3 arguments");}

    let int_1: i32 = args[1].parse().expect("First argument could not be cast as an integer");

    let int_2: i32 = args[2].parse().expect("Second argument could not be cast as an integer");

    let int_3: i32 = args[3].parse().expect("Third argument could not be cast as an integer");

    if int_1 == int_2 && int_2 == int_3 {
      println!("equal");
    } else {
      println!("not equal");
    }
}
