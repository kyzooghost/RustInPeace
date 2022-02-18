fn main() {
  println!{"{}", lnFactorial(5)};
}

fn lnFactorial(n: i32) -> f32 {
  use std::f32;
  if n == 1 {return 0.0}
  else {
    (n as f32).ln() + lnFactorial(n - 1)
  }
}