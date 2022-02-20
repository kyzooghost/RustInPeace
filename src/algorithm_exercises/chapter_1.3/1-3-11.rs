#[allow(non_snake_case)]

trait Stack<T> {
  fn size(&self) -> usize;
  fn isEmpty(&self) -> bool;
  fn push(&mut self, item:T);
  fn pop(&mut self) -> T;
  fn new(cap: usize) -> Self;
  fn top(&self) -> T;
}

#[derive(Debug)]
struct FixedCapacityStackOfI32 {
  N: usize, // size
  a: Vec<i32>, // stack entries
}

impl Stack<i32> for FixedCapacityStackOfI32 {
  fn new(cap: usize) -> Self {
    assert!(cap != 0, "cap cannot be 0");
    FixedCapacityStackOfI32 { N: 0, a: vec![0; cap]}
  }

  fn size(&self) -> usize {
    self.N
  }

  fn isEmpty(&self) -> bool {
    self.N == 0
  }

  fn push(&mut self, item: i32) {
    self.a[self.N] = item;
    self.N = self.N + 1;
  }

  fn pop(&mut self) -> i32 {
    let popped_element: i32 = self.a[self.N - 1].clone();
    self.a[self.N - 1] = 0;
    self.N = self.N - 1;
    popped_element
  }

  fn top (&self) -> i32 {
    self.a[self.N - 1]
  }
}

// 1.3.11
// Write a program EvaluatePostfix that takes a postfix expression from standard
// input, evaluates it, and prints the value. (Piping the output of your program from
// the previous exercise to this program gives equivalent behavior to Evaluate.

fn main() {
  let string1 = String::from("34*25*+");
  assert!( EvaluatePostfix(string1) == 22, "incorrect EvaluatePostFix implementation");

  let string2 = String::from("53+82-*");
  assert!( EvaluatePostfix(string2) == 48, "incorrect EvaluatePostFix implementation");

  println!("Success!");
}

fn EvaluatePostfix(string: String) -> i32 {
  let mut stack = FixedCapacityStackOfI32::new(100);
  let characters: Vec<char> = string.chars().collect();
  
  for c in characters {
    if !isOperator(c) {stack.push(c.to_digit(10).unwrap().try_into().unwrap());}
    else {
      let a:i32 = stack.pop();
      let b:i32 = stack.pop();
      
      if c == '+' {stack.push(b + a);}
      else if c == '-' {stack.push(b - a);}
      else if c == '*' {stack.push(b * a);}
      else if c == '/' {stack.push(b / a);}
      else if c == '^' {stack.push(b ^ a);}
    }
  }

  stack.pop()
}

fn isOperator (character: char) -> bool {
  if character == '+' || character == '-' || character == '*' || character == '/' || character == '^' {return true}
  false
}