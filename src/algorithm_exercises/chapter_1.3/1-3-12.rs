#[allow(non_snake_case)]

trait Stack<T> {
  fn size(&self) -> usize;
  fn isEmpty(&self) -> bool;
  fn push(&mut self, item:T);
  fn pop(&mut self) -> T;
  fn new(cap: usize) -> Self;
  fn top(&self) -> T;
  fn copy (&self) -> Self;
}

#[derive(Debug)]
struct FixedCapacityStackOfStrings {
  N: usize, // size
  a: Vec<String>, // stack entries
}

impl Stack<String> for FixedCapacityStackOfStrings {
  fn new(cap: usize) -> Self {
    assert!(cap != 0, "cap cannot be 0");
    FixedCapacityStackOfStrings { N: 0, a: vec![String::new(); cap]}
  }

  fn size(&self) -> usize {
    self.N
  }

  fn isEmpty(&self) -> bool {
    self.N == 0
  }

  fn push(&mut self, item: String) {
    self.a[self.N] = item;
    self.N = self.N + 1;
  }

  fn pop(&mut self) -> String {
    let popped_element: String = self.a[self.N - 1].clone();
    self.a[self.N - 1] = "".to_string();
    self.N = self.N - 1;
    popped_element
  }

  fn top (&self) -> String {
    self.a[self.N - 1].clone()
  }

  fn copy(&self) -> Self {
    let mut newStack = Self::new(self.N);

    for i in 0..self.N {
      newStack.a[i] = self.a[i].clone();
    }

    newStack
  }
}
// 1.3.12
// Write an iterable Stack client that has a static method copy() that takes a stack
// of strings as argument and returns a copy of the stack. Note : This ability is a prime
// example of the value of having an iterator, because it allows development of such func-
// tionality without changing the basic API.

fn main() {
  let string1 = String::from("34*25*+");
  let stack1 = stringToStack(string1);
  let stack2 = stack1.copy();

  for i in 0..stack1.N {
    assert!(stack1.a[i] == stack2.a[i], "not cloned");
  }

  println!("Success!");
}

fn stringToStack(string: String) -> FixedCapacityStackOfStrings {
  let mut stack = FixedCapacityStackOfStrings::new(100);
  let characters: Vec<char> = string.chars().collect();

  for c in characters {
    stack.push(c.to_string());
  }

  stack
}