// 1.3.10
// Write a filter InfixToPostfix that converts an arithmetic expression from infix to postfix.
// e.g. Infix x^y/(5*z)+2 => Postfix xy^5z*/2+

// Non-operator => Push onto main stack
// Operator => Push onto scratch stack
// 

#[derive(Debug)]
struct FixedCapacityStack<T> {
  N: usize, // size
  a: Vec<T>, // stack entries
}

impl<T> FixedCapacityStack<T> {
  pub fn new(cap: usize) -> Self {
    assert!(cap != 0, "cap cannot be 0");
    FixedCapacityStack { N: 0, a: vec!['\0'; cap]}
  }

  fn size(&self) -> usize {
    self.N
  }

  fn isEmpty(&self) -> bool {
    self.N == 0
  }

  fn push(&mut self, item: char) {
    self.a[self.N] = item;
    self.N = self.N + 1;
  }

  fn pop(&mut self) -> char {
    let popped_element: char = self.a[self.N - 1].clone();
    self.a[self.N - 1] = '\0';
    self.N = self.N - 1;
    popped_element
  }

  fn isFull(&self) -> bool {
    self.N == self.a.len()
  }

}

fn main() {
  // Create stack
  // let string1 = String::from("1+2)*3-4)*5-6)))");
  // let string2 = String::from("((1+2)*((3-4)*(5-6)))");
  // assert!( infix(string1) == string2, "incorrect infix implementation");
  // println!("Successful infix");
}

fn InfixToPostfix(string: String) -> String {
  let mut stack = FixedCapacityStackOfChars::new(100);
  "a".to_string()
}