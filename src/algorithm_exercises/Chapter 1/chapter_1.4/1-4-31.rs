#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.31

// Deque with three stacks. 
// Implement a deque with three stacks so that each deque operation 
// takes a constant amortized number of stack operations.

struct Deque<T> where T: std::fmt::Debug {
    size: usize,
    stack_stack: Vec<T>,
    stack_placeholder: Vec<T>,
    stack_queue: Vec<T>,
}

impl<T> Deque<T> where T: std::fmt::Debug {
    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        Deque {
            size: 0, 
            stack_stack: Vec::new(),
            stack_placeholder: Vec::new(),
            stack_queue: Vec::new()
        }
    }

    pub fn push(&mut self, elem: T) {
        self.size = self.size + 1;
        self.stack_stack.push(elem);
    }

    pub fn enqueue(&mut self, elem: T) {
        self.size = self.size + 1;
        self.stack_queue.push(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            if self.stack_stack.is_empty() {
                // Move half of stack_queue to stack_stack

                for _ in 0..self.size/2 {
                    self.stack_placeholder.push(self.stack_queue.pop().unwrap())
                }

                while !self.stack_queue.is_empty() {
                    self.stack_stack.push(self.stack_queue.pop().unwrap())
                }

                while !self.stack_placeholder.is_empty() {
                    self.stack_queue.push(self.stack_placeholder.pop().unwrap())
                }

            }
            self.size = self.size - 1;
            self.stack_stack.pop()
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            if self.stack_queue.is_empty() {
                // Move half of stack_stack to stack_queue

                for _ in 0..self.size/2 {
                    self.stack_placeholder.push(self.stack_stack.pop().unwrap())
                }

                while !self.stack_stack.is_empty() {
                    self.stack_queue.push(self.stack_stack.pop().unwrap())
                }

                while !self.stack_placeholder.is_empty() {
                    self.stack_stack.push(self.stack_placeholder.pop().unwrap())
                }

            }

            self.size = self.size - 1;    
            self.stack_queue.pop()
        }
    }
}

fn main() {
    let mut a = Deque::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);

    println!("{:?}", a.pop());
    println!("{:?}", a.dequeue());
    println!("{:?}", a.pop());
    println!("{:?}", a.pop());
    println!("{:?}", a.pop());
}