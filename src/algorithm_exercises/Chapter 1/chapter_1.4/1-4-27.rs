#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.27 

// Implement a queue with two stacks so that each queue operation takes 
// a constant amortized number of stack operations. 

// Hint: If you push elements onto a stack and then pop them all, they appear in reverse order. 
// If you repeat this process, they’re now back in order. 

struct Queue<T> where T: std::fmt::Debug {
    size: usize,
    queue_stack: Vec<T>,
    dequeue_stack: Vec<T>
}

impl<T> Queue<T> where T: std::fmt::Debug {
    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        Queue {size: 0, queue_stack: Vec::new(), dequeue_stack: Vec::new()}
    }

    pub fn enqueue(&mut self, elem: T) {
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
}

fn main() {
    let mut a = Queue::new();
    a.enqueue(1);
    a.enqueue(2);
    a.enqueue(3);
    a.enqueue(4);

    a.dequeue();
    a.enqueue(5);
    a.dequeue();
    println!("{:?}", a.queue_stack);
    println!("{:?}", a.dequeue_stack);
    println!("{:?}", a.size);
}