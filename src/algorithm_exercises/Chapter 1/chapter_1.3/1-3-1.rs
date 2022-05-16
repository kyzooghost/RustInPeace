// 1.3.1 
// Add a method isFull()to FixedCapacityStackOfStrings

#[derive(Debug)]
struct FixedCapacityStackOfStrings {
  N: usize, // size
  a: Vec<String>, // stack entries
}

impl FixedCapacityStackOfStrings {
  pub fn new(cap: usize) -> Self {
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
    let popped_element: String = self.a[self.N].clone();
    self.a[self.N] = "".to_string();
    self.N = self.N - 1;
    popped_element
  }

  fn isFull(&self) -> bool {
    self.N == self.a.len()
  }
}

fn main() {
  let mut x = FixedCapacityStackOfStrings::new(2);
  assert!(!x.isFull(), "Stack is expected to be not full");
  x.push("a".to_string());
  x.push("b".to_string());
  assert!(x.isFull(), "Stack is expected to be full");
}
