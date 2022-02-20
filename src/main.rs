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

// 1.3.9
// Write a program that takes from standard input an expression without left parentheses and prints the equivalent infix expression with the parentheses inserted. 
// For example, given the input: 1 + 2 ) * 3 - 4 ) * 5 - 6 ) ) )
// Your program should print ( ( 1 + 2 ) * ( ( 3 -4 ) * ( 5 - 6 ) ) )

fn main() {
  let string1 = String::from("1+2)*3-4)*5-6)))");
  let string2 = String::from("((1+2)*((3-4)*(5-6)))");
  assert!( infix(string1) == string2, "incorrect infix implementation");
  // println!("{}", infix(string1));
  println!("Successful infix");
}

fn infix(string: String) -> String {
  let mut stack = FixedCapacityStackOfChars::new(100);
  let mut scratchStack = FixedCapacityStackOfChars::new(100);
  let characters: Vec<char> = string.chars().collect();
  
  for c in characters {
    // Push to stack if character is letter or number
    if !isParenthesis(c) {stack.push(c);}

    else if c == ')' {
        let mut parenthesisCount: i32 = 0;
        let mut operatorCount: i32 = 0;

        while operatorCount < 2 {          
          
          // Pop everything from scratchStack onto stack
          if stack.isEmpty() {
            stack.push('(');
            while !scratchStack.isEmpty() {
               stack.push(scratchStack.pop());
            }
            stack.push(')');
            break;
          }
          
          else if stack.top() == ')' {parenthesisCount = parenthesisCount + 1;}
          else if stack.top() == '(' {parenthesisCount = parenthesisCount - 1;}
          else if isOperator(stack.top()) && parenthesisCount == 0 {operatorCount = operatorCount + 1;}

          // Pop everything from scratchStack onto stack
          if operatorCount == 2 {
            stack.push('(');
            while !scratchStack.isEmpty() {
               stack.push(scratchStack.pop());
            }
            stack.push(')');
            break;
          }

          scratchStack.push(stack.pop());
        }
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