#[allow(non_snake_case)]

trait Stack<T> {
  fn size(&self) -> usize;
  fn isEmpty(&self) -> bool;
  fn push(&mut self, item:T);
  fn pop(&mut self) -> T;
  fn new(cap: usize) -> Self;
  fn toString(&mut self) -> String;
}

#[derive(Debug)]
struct FixedCapacityStackOfStrings {
  N: usize, // size
  a: Vec<String>, // stack entries
}

#[derive(Debug)]
struct FixedCapacityStackOfChars {
  N: usize, // size
  a: Vec<char>, // stack entries
}

impl Stack<char> for FixedCapacityStackOfChars {
  fn new(cap: usize) -> Self {
    assert!(cap != 0, "cap cannot be 0");
    FixedCapacityStackOfChars { N: 0, a: vec!['\0'; cap]}
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

  fn toString(&mut self) -> String {
    let mut string = String::from("");
    while !self.isEmpty() {
      string.push(self.pop());
    }
    string.chars().rev().collect::<String>()
  }
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

  fn toString(&mut self) -> String {
    let mut string = String::from("");
    while !self.isEmpty() {
      string.push(self.pop().chars().nth(0).expect("?Not a single character string"));
    }
    string.chars().rev().collect::<String>()
  }
}

// 1.3.10
// Write a filter InfixToPostfix that converts an arithmetic expression from infix to postfix.
// e.g. Infix x^y/(5*z)+2 => Postfix xy^5z*/2+

// Non-operator => Push onto main stack
// Operator => Push onto scratch stack
// Every 2 non-operator -> push operator onto stack

fn main() {
  // Create stack
  let string1 = String::from("x^y/(5*z)+2");
  let string2 = String::from("xy^5z*/2+");
  assert!( InfixToPostfix(string1) == string2, "incorrect InfixToPostfix implementation");
  println!("Success!");
}

fn InfixToPostfix(string: String) -> String {
  let mut stack = FixedCapacityStackOfChars::new(100);
  let mut operatorStack = FixedCapacityStackOfChars::new(100);
  let characters: Vec<char> = string.chars().collect();
  let mut parenthesisCount:usize = 0;

  for c in characters {
    // If variable, push onto stack
    if !isOperator(c) && !isParenthesis(c) {
      stack.push(c);

      if operatorStack.N > parenthesisCount {stack.push(operatorStack.pop())}
      continue;
    }

    if c == '(' {
      parenthesisCount = parenthesisCount + 1;
      continue;
    }

    if c == ')' {
      stack.push(operatorStack.pop());
      parenthesisCount = parenthesisCount - 1;
      continue;
    }

    if isOperator(c) {
      operatorStack.push(c);
      continue;
    }
  }

  stack.toString()
}

fn isOperator (character: char) -> bool {
  if character == '+' || character == '-' || character == '*' || character == '/' || character == '^' {return true}
  false
}

fn isParenthesis (character: char) -> bool {
  if character == '(' || character == ')' {return true}
  false
}