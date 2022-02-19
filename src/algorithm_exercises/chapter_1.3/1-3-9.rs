// 1.3.9
// Write a program that takes from standard input an expression without left parentheses and prints the equivalent infix expression with the parentheses inserted. 
// For example, given the input: 1 + 2 ) * 3 - 4 ) * 5 - 6 ) ) )
// Your program should print ( ( 1 + 2 ) * ( ( 3 -4 ) * ( 5 - 6 ) ) )

#[derive(Debug)]
struct FixedCapacityStackOfChars {
  N: usize, // size
  a: Vec<char>, // stack entries
}

impl FixedCapacityStackOfChars {
  pub fn new(cap: usize) -> Self {
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

  fn isFull(&self) -> bool {
    self.N == self.a.len()
  }

  fn insertElementIntoStack(&mut self, index: usize, new_element: char) {
    for i in (index..self.N).rev() {
      // Start from the top element
      // Push only the top element
      if i == self.N-1 {self.push(self.a[i]);}
      // Re-assign each stack element i, to slot i + 1
      else {self.a[i + 1] = self.a[i]}
    }

    // Insert new_element into a[index]
    self.a[index] = new_element;
  }

  // Given an index for closing parenthesis in the stack, find the index to insert the correlating opening parenthesis
  // If no position found, returns 0 to insert parenthesis at the start
  fn findIndexToInsertOpeningParenthesis(&self, index: usize) -> usize {
    let mut operatorCount: i32 = 0;
    let mut parenthesisCount: i32 = 0;
    // println!("{}", index);

    for i in (0..index).rev() {
      // Start from the top of stack
      // Search down the stack for operator element
      // Want to insert opening parenthesis one index above the second operator found
      // Need to make operatorCount immutable, when scanning closed parenthesis block
      // Use mutex concept

      if self.a[i] == ')'  { parenthesisCount = parenthesisCount + 1;}
      if self.a[i] == '('  { parenthesisCount = parenthesisCount - 1;}

      if parenthesisCount == 0 && 
      (self.a[i] == '*' || self.a[i] == '+' || self.a[i] == '-' || self.a[i] == '/') {
        operatorCount = operatorCount + 1;
      }

      if operatorCount == 2 {
        return i + 1
      }
    }

    // If operator was not found, then we want to return the bottom of stack to insert opening parenthesis at
    0
  }

  fn toString(&mut self) -> String {
    let mut string = String::from("");
    while !self.isEmpty() {
      string.push(self.pop());
    }
    string.chars().rev().collect::<String>()
  }
}

fn main() {
  // Create stack
  let string1 = String::from("1+2)*3-4)*5-6)))");
  let string2 = String::from("((1+2)*((3-4)*(5-6)))");
  assert!( infix(string1) == string2, "incorrect infix implementation");
  println!("Successful infix");
}

fn infix(string: String) -> String {
  let mut stack = FixedCapacityStackOfChars::new(100);
  let characters: Vec<char> = string.chars().collect();
  for c in characters {stack.push(c);}

  // From bottom of stack (is this illegal lol? Meh fuck it)
  let mut skipNext: bool = false;
  let mut i = 0;

  while i < stack.N {
    i = i + 1;

    if stack.a[i] == ')' {
      if (skipNext) {
        skipNext = false;
        continue;
      }

      let openingParenthesisIndex = stack.findIndexToInsertOpeningParenthesis(i);
      stack.insertElementIntoStack(openingParenthesisIndex, '(');
      skipNext = true;
    }
  }

  stack.toString()
}