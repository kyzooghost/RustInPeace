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
    let popped_element: String = self.a[self.N - 1].clone();
    self.a[self.N - 1] = "".to_string();
    self.N = self.N - 1;
    popped_element
  }

  fn isFull(&self) -> bool {
    self.N == self.a.len()
  }
}

// 1.3.9
// Write a program that takes from standard input an expression without left parentheses and prints the equivalent infix expression with the parentheses inserted. 
// For example, given the input: 1 + 2 ) * 3 - 4 ) * 5- 6 ) ) )
// Your program should print ( ( 1 + 2 ) * ( ( 3 -4 ) * ( 5 - 6 ) )

fn main() {
  let string1 = String::from("1+2)*3-4)*5-6)))");
  let string2 = String::from("((1+2)*((3-4)*(5-6))");
  assert!( infex(string1) == string2, "incorrect infix implementation");
}

fn infix(string: String) -> bool {
  let mut stack = FixedCapacityStackOfStrings::new(100);
  let characters: Vec<char> = string.chars().collect();

  // Input check
  for c in characters.clone() {
    assert!( c == '[' || c == '{' || c == '(' || c == ']' || c == '}' || c == ')' 
    , "not a parenthesis character"
    )
  }

  // Push if [, (, {
  for c in characters {
    // println!{"{}", c};
    if c == '[' || c == '{' || c == '(' {
      stack.push(c.to_string());
    } else if c == ']' {
        // println!("{:?}", stack.pop());
        if stack.pop() != "[" {return false}
    } else if c == ')' {
        if stack.pop() != "(" {return false}
    } else if c == '}' {
        if stack.pop() != "{" {return false}
    }
  }

  true
}