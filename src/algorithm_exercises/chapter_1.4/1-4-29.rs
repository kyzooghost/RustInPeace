#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.29

// Implement a steque with two stacks so that each steque operation (see Exercise 1.3.32) 
// takes a constant amortized number of stack operations.

// Steque - push, pop and enqueue
struct Steque<T> where T: std::fmt::Debug {
    size: usize,
    queue_stack: Vec<T>,
    dequeue_stack: Vec<T>
}

impl<T> Steque<T> where T: std::fmt::Debug {
    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        Steque {size: 0, queue_stack: Vec::new(), dequeue_stack: Vec::new()}
    }

    pub fn push(&mut self, elem: T) {
        self.queue_stack.push(elem);
        self.size = self.size + 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            self.size = self.size - 1;

            if self.dequeue_stack.is_empty() {
                while !self.queue_stack.is_empty() {
                    self.dequeue_stack.push(self.queue_stack.pop().unwrap())
                }
            }
            
            self.dequeue_stack.pop()
        }
    }

    // Enqueue in this case, means that what you enqueue, will be popped last
    pub fn enqueue(&mut self, elem: T) {
        self.dequeue_stack.push(elem);
        self.size = self.size + 1;
    }
}

fn main() {
    let mut a = Steque::new();
    a.push(1);
}