#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.30

// Implement a deque with a stack and a steque (see Exercise 1.3.32) 
// so that each deque operation takes a constant amortized number of stack and steque operations.

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

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            self.size = self.size - 1;

            if self.queue_stack.is_empty() {
                while !self.dequeue_stack.is_empty() {
                    self.queue_stack.push(self.dequeue_stack.pop().unwrap())
                }
            }
            
            self.queue_stack.pop()
        }
    }

    // Enqueue in this case, means that what you enqueue, will be popped last
    pub fn enqueue(&mut self, elem: T) {
        self.dequeue_stack.push(elem);
        self.size = self.size + 1;
    }
}

// Deque has dequeue operation
// Push and enqueue can be the same
// Can pop from steque
// Dequeue = pop from stack
// Half in steque, half in 

// Push and pop from steque
// Enqueue and dequeue from stack
// With each pop and dequeue operation, if one side is empty, transfer half over

struct Deque<T> where T: std::fmt::Debug {
    size: usize,
    stack: Vec<T>,
    steque: Steque<T>
}

impl<T> Deque<T> where T: std::fmt::Debug {
    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        Deque {size: 0, stack: Vec::new(), steque: Steque::new()}
    }

    pub fn push(&mut self, elem: T) {
        self.size = self.size + 1;
        self.steque.push(elem)
    }

    pub fn enqueue(&mut self, elem: T) {
        self.size = self.size + 1;
        self.stack.push(elem)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            self.size = self.size - 1;

            if self.steque.size == 0 {
                while !self.stack.is_empty() {
                    self.steque.push(self.stack.pop().unwrap())
                }
            }

            self.steque.pop()
        }
    }

// 1 2 3 4

// 3 4
// 2 1

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            if self.stack.len() == 0 {
                let size_to_remain_on_steque = self.size / 2;
                let size_to_shift_to_stack = self.size - self.size / 2; // In case self.size is an odd number

                for _ in 0..size_to_remain_on_steque {
                    let mut placeholder_vec = Vec::new(); // Need placeholder vec because cannot perform memory mutation on the same data structure in the one expression twice, even though we are performing mutation operations on two distinct parts within the data structure (the queue_stack and dequeue_stack)
                    println!("{:?}", placeholder_vec);
                    placeholder_vec.push(self.steque.pop().unwrap());
                    self.steque.enqueue(placeholder_vec.pop().unwrap());
                }

                for _ in 0..size_to_shift_to_stack{
                    self.stack.push(self.steque.pop().unwrap())
                }
            }

            self.size = self.size - 1;    
            self.stack.pop()
        }
    }
}

fn main() {
    let mut a = Deque::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    a.push(5);
    println!("{:?}", a.dequeue());
    println!("{:?}", a.dequeue());
    println!("{:?}", a.pop());
    println!("{:?}", a.pop());
    println!("{:?}", a.pop());
}