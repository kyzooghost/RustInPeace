#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    vec: Vec<T>
}

impl<T> Stack<T> {
    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        Stack {size: 0, vec: Vec::new()}
    }

    pub fn push(&mut self, elem: T) {
        self.vec.push(elem);
        self.size = self.size + 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            self.size = self.size - 1;
            self.vec.pop()
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.vec.last()
    }
}

// 1.3.45 Stack generability. Suppose that we have a sequence of intermixed push and pop operations as with our test stack client, 
// where the integers 0, 1, ..., N-1 in that order (push directives) are intermixed with N minus signs (pop directives). 
// Devise an algorithm that determines whether the intermixed sequence causes the stack to underflow. 
// (You may use only an amount of space independent of N — you cannot store the integers in a data structure.) 

// Devise a linear-time algorithm that determines whether a given permutation can be generated as output by our test client 
// (depending on where the pop directives occur).

// Sort of no point doing this with our Stack implemention, we are spoilt with pop() returning an Option
// -> If pop() returns None, that means underflow

// Collect push and pop directives in two separate arrays
// If pop directives array size ever > push directives array size => underflow


fn main() {
    let input1Values = String::from("0 1 2 - - -");
    println!("{:?}", willTheStackUnderflow(input1Values));

    let input2Values = String::from("0 1 2 - - - 3 4 5 - - 6 - - -");
    println!("{:?}", willTheStackUnderflow(input2Values));

    let permutation1 = String::from("4 3 2 1 0 9 8 7 6 5");
    println!("{:?}", canAPermutationBeGenerated(permutation1));

    let permutation2 = String::from("4 6 8 7 5 3 2 9 0 1");
    println!("{:?}", canAPermutationBeGenerated(permutation2));
}

fn willTheStackUnderflow(inputValues: String) -> bool {
    let mut itemsCount:i32 = 0;
    
    for value in inputValues.split_whitespace() {
        if value == "-" {itemsCount = itemsCount - 1;}
        else {itemsCount = itemsCount + 1;}

        if itemsCount < 0 {return true}
    }

    false
}

fn canAPermutationBeGenerated(inputValues: String) -> bool {
    let mut stack = Stack::new();
    let mut current = 0;

    for value in inputValues.split_whitespace() {
        
        let integerValue = value.parse::<i32>().unwrap();
        println!("{:?}", integerValue);
        println!("{:?}", stack);

        if stack.isEmpty() || &integerValue > stack.peek().unwrap() {
            while current < integerValue {
                stack.push(current);
                current = current + 1;
            }

            current = current + 1;
        } else if &integerValue == stack.peek().unwrap() {
            stack.pop();
        } else {
            return false
        }
    }

    true
}