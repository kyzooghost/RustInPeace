fn main() {
  let mut fibonacci_vec = vec![];
  let N:i32 = 50;

  for x in 0..N {
    fibonacci_vec.push(F(x));
  }

  println!{"{:?}", fibonacci_vec};

  
}

fn F(N: i32) -> i32 {
  if N == 0 {return 0}
  if N == 1 {return 1}
  return F(N - 1) + F(N - 2)
}
