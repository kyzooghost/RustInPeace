#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.3.9
// Write a program that takes from standard input an expression without left parentheses and prints the equivalent infix expression with the parentheses inserted. 
// For example, given the input: 1 + 2 ) * 3 - 4 ) * 5 - 6 ) ) )
// Your program should print ( ( 1 + 2 ) * ( ( 3 -4 ) * ( 5 - 6 ) ) )

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

fn main() {
  let string1 = String::from("1 + 2 ) * 3 - 4 ) * 5 - 6 ) ) ) ");
  let string2 = String::from("((1 + 2) * ((3 - 4) * (5 - 6)))");
  assert!( infix(string1) == string2, "incorrect infix implementation");
  println!("Successful infix");
}

fn infix(input: String) -> String {
  let mut operands = FixedCapacityStackOfStrings::new(100);
  let mut operators = FixedCapacityStackOfStrings::new(100);

  let mut inputIterator = input.split_whitespace();

  for value in inputIterator {
    if value.eq("(") {
      // do nothing

    } else if value.eq("+") || value.eq("-") || value.eq("*") || value.eq("+") {
      operators.push(value.to_string());

    } else if value.eq(")") {
      let operator = operators.pop();
      let value2 = operands.pop();
      let value1 = operands.pop();

      let subExpression = format!("{}{}{}{}{}{}{}", 
        String::from("("), 
        value1,
        String::from(" "),
        operator,
        String::from(" "),
        value2,
        String::from(")"),
      );

      operands.push(subExpression);

    } else {
      operands.push(value.to_string());
    }
  }

  operands.pop()
}