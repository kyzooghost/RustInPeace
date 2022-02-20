#[allow(non_snake_case)]

trait Stack<T> {
  fn size(&self) -> usize;
  fn isEmpty(&self) -> bool;
  fn push(&mut self, item:T);
  fn pop(&mut self) -> T;
  fn new(cap: usize) -> Self;
  fn top(&self) -> T;
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

  fn top (&self) -> char {
    self.a[self.N - 1]
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

  fn top (&self) -> String {
    self.a[self.N - 1].clone()
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

// https://www.tutorialspoint.com/Convert-Infix-to-Postfix-Expression

fn main() {
  let string1 = String::from("x^y/(5*z)+2");
  let string2 = String::from("xy^5z*/2+");
  assert!( InfixToPostfix(string1) == string2, "incorrect InfixToPostfix implementation");

  let string3 = String::from("a+b*(c^d-e)^(f+g*h)-i");
  let string4 = String::from("abcd^e-fgh*+^*+i-");
  assert!( InfixToPostfix(string3) == string4, "incorrect InfixToPostfix implementation");

  println!("Success!");
}

fn InfixToPostfix(string: String) -> String {
  let mut postfixStack = FixedCapacityStackOfChars::new(100);
  let mut operatorStack = FixedCapacityStackOfChars::new(100);
  let infix: Vec<char> = string.chars().collect();
  let mut parenthesisCount:usize = 0;

  for c in infix {
    // Push to postfixStack if character is letter or number
    if !isOperator(c) && !isParenthesis(c) {postfixStack.push(c);}

    // If (, push to operatorStack
    else if c == '(' {operatorStack.push(c);}

    else if c == ')' {
      while !operatorStack.isEmpty() && operatorStack.top() != '(' {
        // Pop everything but ( from operatorStack into postfixStack
        postfixStack.push(operatorStack.pop());
      }
      // Remove ( from operatorStack
      operatorStack.pop(); 
    }

    else {
      // If nothing on operatorStack, push onto it 
      if operatorStack.isEmpty() {operatorStack.push(c)}
      // If bigger precedence, push onto operator stack
      else if precedence(c) > precedence(operatorStack.top()) {
        operatorStack.push(c);
      } else {
        // else pop operatorStack onto postfixStack, until bigger precedence
          while !operatorStack.isEmpty() && precedence(c) <= precedence (operatorStack.top()) {
            postfixStack.push(operatorStack.pop());     
          }
        operatorStack.push(c);
      }
    } 
  }

  while !operatorStack.isEmpty() {
    postfixStack.push(operatorStack.pop());
  }

  postfixStack.toString()
}

fn isOperator (character: char) -> bool {
  if character == '+' || character == '-' || character == '*' || character == '/' || character == '^' {return true}
  false
}

fn isParenthesis (character: char) -> bool {
  if character == '(' || character == ')' {return true}
  false
}

fn precedence(operator: char) -> i32 {
  if operator == '+' || operator == '-' {return 1}
  else if operator == '*' || operator == '/' {return 2}
  else if operator == '^' {return 3}
  else {return 0}
}