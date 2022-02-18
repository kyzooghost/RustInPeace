// Give the sequence of values of p and q that are computed when Euclid’s algorithm
// is used to compute the greatest common divisor of 105 and 24. Extend the code
// given on page 4 to develop a program Euclid that takes two integers from the command
// line and computes their greatest common divisor, printing out the two arguments for
// each call on the recursive method. Use your program to compute the greatest common
// divisor or 1111111 and 1234567.

fn main() {
  println!{"{}", gcd(1111111, 1234567, 0)};
}

fn gcd(p:i32, q:i32, _depth:i32) -> i32 {

  let mut indent = String::from("");

  for _i in 0.._depth {
    indent.push_str(" ");
  }

  println!("{}p: {}q: {}",indent, p, q);

  if q == 0 {
    return p
  } else {
    let r: i32 = p % q;
    return gcd (q, r, _depth + 1)
  }

}